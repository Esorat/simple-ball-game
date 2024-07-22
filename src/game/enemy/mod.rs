use bevy::prelude::*;
pub mod components;
pub mod resources;
mod systems;

use resources::*;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const NUMBER_OF_ENEMIES: usize = 4; //numbers of spawn enemys.
pub const ENEMY_SPEED: f32 = 100.0; //enemy speed.
pub const ENEMY_SIZE: f32 = 64.0; //enemy size.

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        //Resouces
        app.init_resource::<EnemySpawnTimer>()
        //Startup Sustems 
            // .add_startup_system(spawn_enemies)
                //Enter State Systems
            .add_system(spawn_enemies.in_schedule(OnEnter(AppState::Game)))
            //Systems
            .add_system(enemy_movement
                .run_if(in_state(AppState::Game))
                .run_if(in_state(SimulationState::Running)),)
            // .add_system(enemy_movement)
            // .add_system(update_enemy_direction)
            // .add_system(confine_enemy_movement)
            // .add_system(tick_enemy_spawn_timer)
            // .add_system(spawn_enemies_over_time)

        .add_systems(
            (
                enemy_movement,
                update_enemy_direction,
                confine_enemy_movement,
                tick_enemy_spawn_timer,
                spawn_enemies_over_time,
            )
            .in_set(OnUpdate(AppState::Game))
            .in_set(OnUpdate(SimulationState::Running))
        )
        //Exit State System;
    .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }

}
