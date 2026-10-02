use std::path::Path;

use bevy::prelude::*;

use crate::enemies::basic::WalkNodes;
use crate::enemies::types::EnemyType;
use crate::map::Map;
use crate::map::objectives::Spawner;

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawner: Res<Spawner>,
) {
    let enem = EnemyType::Basic;
    let map = Map::from_map_file(Path::new("./levels/level0/level0.map"));
    let walknodes =
        WalkNodes::from_path_file(Path::new("./levels/level0/path0.path"), &map).unwrap();
    commands.spawn(enem.create_enemy(walknodes, spawner.position, &mut meshes, &mut materials));
}
