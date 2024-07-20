use bevy::prelude::*;

pub mod componets;
mod systems;

use systems::*;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct MovementSystemSet;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ConfineSystemSet;

pub struct PlayerPlugin;


impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
      
        //Startup system
        app.configure_set(MovementSystemSet.before(ConfineSystemSet))
        .add_startup_system(spawn_player)
            //Systems
            .add_system(player_movement.in_set(MovementSystemSet))
            .add_system(confine_player_movement.in_set(ConfineSystemSet))
            .add_system(enemy_hit_player)
            .add_system(player_hit_star);
    }
}
