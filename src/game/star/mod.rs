use bevy::prelude::*;

pub mod componets;
mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;
use super::SimulationState;

pub const NUMBER_OF_STARS: usize = 10; //number spawn stars.
pub const STAR_SIZE: f32 = 30.0; //star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
        //On Enter State
        .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
        //Systems
            .add_systems((
                tick_spawn_timer,
                spawn_stars_over_time,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )

        //On System State
        .add_system(despawn_stars.in_schedule(OnExit(AppState::Game)));
    }
}
