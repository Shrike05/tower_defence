use crate::enemies::{movement::update_enemy_positions, spawner::*};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
        app.add_systems(Update, (update_enemy_positions, game_timer));
    }
}
