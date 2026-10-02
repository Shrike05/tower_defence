use crate::enemies::{movement::update_enemy_positions, spawner::*};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
        app.add_systems(Update, update_enemy_positions);
    }
}
