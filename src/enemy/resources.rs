use bevy::prelude::*;

pub const SPAWN_ENEMY_TIMER: f32 = 5.0; // spawn enemy timer.

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> EnemySpawnTimer {
        EnemySpawnTimer {
            timer: Timer::from_seconds(SPAWN_ENEMY_TIMER, TimerMode::Repeating),
        }
    }
}
