use bevy::prelude::*;

mod camera;
mod enemies;
mod map;
mod towers;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            map::MapPlugin,
            camera::CameraPlugin,
            enemies::EnemyPlugin,
            towers::TowerPlugin,
        ))
        .run();
}
