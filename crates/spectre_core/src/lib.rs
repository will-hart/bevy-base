use bevy::prelude::*;

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
    pub amount: u16,
}

pub struct BuffableStatistic {
    pub base_value: u16,
    pub value: u16,
    pub buffs: Vec<Buff>,
}

impl BuffableStatistic {
    pub fn new(base_value: u16) -> Self {
        BuffableStatistic {
            base_value,
            value: base_value,
            buffs: Vec::default(),
        }
    }

    pub fn set_base(&mut self, new_base: u16) {
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
        let (abs, perc) = self.buffs[..]
            .into_iter()
            .fold((0 as u16, 0. as f32), |acc, buff| {
                return (acc.0 + buff.amount, acc.1 + buff.percentage);
            });

        self.value = (self.base_value as f32 * (1.0 + perc)).floor() as u16 + abs;
    }
}

#[derive(Bundle)]
pub struct CharacterStats {
    pub stats: Stats,
    pub health: Health,
    pub movement: Movement,
}

pub struct Stats {
    pub strength: BuffableStatistic,
    pub agility: BuffableStatistic,
    pub intelligence: BuffableStatistic,
}

pub struct Health {
    pub max_health: BuffableStatistic,
    pub current_health: u16,
    pub target_health: u16,
}

impl Health {
    pub fn new(health: u16) -> Self {
        Health {
            max_health: BuffableStatistic::new(health),
            current_health: health,
            target_health: health,
        }
    }
}

pub struct Movement {
    pub movement_speed: BuffableStatistic,
}
