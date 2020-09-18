use bevy::prelude::*;

/// A resource which stores the current game speed and elapsed game time
pub struct GameTime {
    pub game_speed: f32,
    pub elapsed_time: f32,
}

impl Default for GameTime {
    fn default() -> GameTime {
        GameTime {
            game_speed: 0.0,
            elapsed_time: 0.0,
        }
    }
}

impl GameTime {
    /// Returns true if the game is currently paused
    pub fn is_paused(&self) -> bool {
        self.game_speed < 0.01
    }
}

/// Add this on a new entity (with no other components) to request a game speed change
pub struct GameSpeedRequest {
    pub new_game_speed: f32,
}

impl GameSpeedRequest {
    pub fn new(speed: f32) -> Self {
        GameSpeedRequest {
            new_game_speed: speed,
        }
    }
}

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<GameTime>()
            .add_stage_before("update", "game_timer")
            .add_system_to_stage("game_timer", game_speed_update.system())
            .add_system_to_stage("game_timer", game_timer.system());
    }
}

fn game_speed_update(
    mut commands: Commands,
    mut game_time: ResMut<GameTime>,
    mut query: Query<(Entity, &GameSpeedRequest)>,
) {
    for (entity, game_speed) in &mut query.iter() {
        println!(
            "Changing game speed from {} to {}",
            game_time.game_speed, game_speed.new_game_speed
        );
        game_time.game_speed = game_speed.new_game_speed;

        commands.despawn(entity);
    }
}

fn game_timer(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    let delta = time.delta_seconds * game_time.game_speed;

    if delta < 0.01 {
        return;
    }

    game_time.elapsed_time += delta;
}
