use bevy::prelude::*;

mod camera;
mod enemies;
mod map;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            map::MapPlugin,
            camera::CameraPlugin,
            enemies::EnemyPlugin,
        ))
        .run();
}

// TODO:
// Setup Grid with backdrop and objectives
// Spawn enemies
// Let player place towers/operators
