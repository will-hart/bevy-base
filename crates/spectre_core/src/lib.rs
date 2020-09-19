use bevy::prelude::*;
use spectre_time::*;

pub mod prelude {
    pub use crate::*;
}

/// Contains a description of a stat buff
/// set expiry to a game time to automatically remove at that time.
/// set expiry to 0 to never expire
/// set either percentage (0.1 for 10% buff, percentages add don't stack) or an absolute increase in amount.
/// if both percentage and amount are non zero, then both will be used
/// absolute buffs are added after percentage buffs, the formula is:
///     value = base_value * (1 + sum of percentages) + sum of amounts
pub struct Buff {
    pub expiry: f32,
    pub percentage: f32,
    pub amount: f32,
}

pub struct BuffableStatistic {
    pub base_value: f32,
    pub value: f32,
    pub buffs: Vec<Buff>,
}

impl BuffableStatistic {
    pub fn new(base_value: f32) -> Self {
        BuffableStatistic {
            base_value,
            value: base_value,
            buffs: Vec::default(),
        }
    }

    pub fn set_base(&mut self, new_base: f32) {
        self.base_value = new_base;
        self.recalculate();
    }

    pub fn update(&mut self, game_time: f32) {
        // remove old buffs
        let len = self.buffs.len();
        if len == 0 {
            return;
        }

        {
            self.buffs
                .retain(|buff| (buff.expiry - 0.).abs() < 0.05 || buff.expiry < game_time);
        }

        if self.buffs.len() != len {
            self.recalculate();
        }
    }

    fn recalculate(&mut self) {
        let (abs, perc) = self.buffs[..].into_iter().fold((0., 0.), |acc, buff| {
            return (acc.0 + buff.amount, acc.1 + buff.percentage);
        });

        self.value = (self.base_value as f32 * (1.0 + perc)).floor() + abs;
    }
}

#[derive(Bundle)]
pub struct CharacterStats {
    pub stats: Stats,
    pub movement: Movement,
    pub health: Health,
    pub mana: Mana,
}

pub struct Stats {
    pub strength: BuffableStatistic,
    pub agility: BuffableStatistic,
    pub intelligence: BuffableStatistic,

    /// A flag that triggers updating child stats when this is update
    pub is_changed: bool,
}

pub struct Health {
    pub max_health: BuffableStatistic,
    pub current_health: f32,
    pub target_health: f32,
    pub regeneration: f32,
}

impl Health {
    pub fn new(health: f32) -> Self {
        Health {
            max_health: BuffableStatistic::new(health),
            current_health: health,
            target_health: health,
            regeneration: 3.,
        }
    }
}

pub struct Mana {
    pub max_mana: BuffableStatistic,
    pub current_mana: f32,
    pub regeneration: f32,
}

impl Mana {
    pub fn new(mana: f32) -> Self {
        Mana {
            max_mana: BuffableStatistic::new(mana),
            current_mana: mana,
            regeneration: 2.,
        }
    }
}

pub struct Movement {
    pub movement_speed: BuffableStatistic,
}

pub struct CharacterStatsPlugin;

impl Plugin for CharacterStatsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_stage_before("update", "update_stats")
            .add_system_to_stage("update_stats", refresh_movement.system())
            .add_stage_after("update", "regeneration")
            .add_system_to_stage("regeneration", health_regeneration.system())
            .add_system_to_stage("regeneration", mana_regeneration.system());
    }
}

fn refresh_movement(
    mut stats: Mut<Stats>,
    mut movement: Mut<Movement>,
    mut health: Mut<Health>,
    mut mana: Mut<Mana>,
) {
    if !stats.is_changed {
        return;
    }

    movement
        .movement_speed
        .set_base(stats.agility.base_value * 10.);
    health.max_health.set_base(stats.strength.base_value * 10.);
    mana.max_mana.set_base(stats.intelligence.base_value * 10.);

    stats.is_changed = false;
}

fn health_regeneration(time: Res<GameTime>, mut health: Mut<Health>) {
    if health.current_health < 0.5 {
        // don't regen when dead
        return;
    }

    health.target_health += health.regeneration * time.delta;

    // check target isn't above max
    if health.target_health > health.max_health.value {
        health.target_health = health.max_health.value;
    }

    // clamp health to maximum
    if health.current_health > health.max_health.value {
        health.current_health = health.max_health.value;
    } else if health.current_health < 0. {
        health.current_health = 0.;
    }
}

fn mana_regeneration(time: Res<GameTime>, mut mana: Mut<Mana>) {
    mana.current_mana = mana.regeneration * time.delta;

    // clamp health to maximum
    if mana.current_mana > mana.max_mana.value {
        mana.current_mana = mana.max_mana.value;
    } else if mana.current_mana < 0. {
        mana.current_mana = 0.;
    }
}
