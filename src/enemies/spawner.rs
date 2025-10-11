use bevy::prelude::*;

use crate::enemies::enemy::EnemyType;
use crate::map::objectives::Spawner;

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    spawner: Res<Spawner>
) {
    let enem = EnemyType::Basic;
    commands.spawn(enem.create_enemy(spawner.position, &mut meshes, &mut materials));
}           
