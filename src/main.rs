mod map;
mod camera;
mod enemies;

use bevy::prelude::*;

use crate::enemies::enemy::EnemyPlugin;
use crate::map::map::MapPlugin;
use crate::camera::camera::CameraPlugin;


fn main() {
    App::new().add_plugins((
        DefaultPlugins,
        MapPlugin,
        CameraPlugin,
        EnemyPlugin
    ))
    .run();
}


// TODO: 
// Setup Grid with backdrop and objectives
// Spawn enemies
// Let player place towers/operators