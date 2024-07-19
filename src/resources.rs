
use bevy::prelude::*;


pub const STAR_SPAWN_TIMER: f32 = 1.0; //spawn stars timer.
pub const SPAWN_ENEMY_TIMER: f32 = 5.0; // spawn enemy timer.


#[derive(Resource)]
pub struct Score {
    pub value: u32, 
}

impl Default for Score {
    fn default() -> Score {
        Score { value: 0 }
    }
}

#[derive(Resource, Debug)]
pub struct HighScores {
    pub scores: Vec<(String, u32)>
}

impl Default for HighScores {
    fn default() -> HighScores {
      HighScores {
        scores: Vec::new()
      }
    }
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> StarSpawnTimer {
        StarSpawnTimer {
            timer: Timer::from_seconds(STAR_SPAWN_TIMER, TimerMode::Repeating),
        }
    }
}

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
