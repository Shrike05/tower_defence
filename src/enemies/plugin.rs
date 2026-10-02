use std::path::Path;

use crate::{
    enemies::{
        basic::WalkNodes,
        movement::{update_enemy_positions, update_enemy_progress},
        spawner::*,
    },
    map::Map,
};
use bevy::prelude::*;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        let map = Map::from_map_file(Path::new("./levels/level0/level0.map"));
        let walknodes =
            WalkNodes::from_path_file(Path::new("./levels/level0/path0.path"), &map).unwrap();
        app.insert_resource(walknodes);
        app.add_systems(Startup, spawn_enemy);
        app.add_systems(Update, (update_enemy_positions, update_enemy_progress));
    }
}
