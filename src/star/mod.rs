use bevy::prelude::*;

pub mod componets;
mod resources;
mod systems;

use systems::*;
use resources::*;

pub const NUMBER_OF_STARS: usize = 10; //number spawn stars.
pub const STAR_SIZE: f32 = 30.0; //star sprite size.

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_stars)
            .add_system(tick_spawn_timer)
            .add_system(spawn_stars_over_time)
            .init_resource::<StarSpawnTimer>();
    }
}
