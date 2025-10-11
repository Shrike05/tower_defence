use bevy::prelude::*;

use crate::enemies::spawner::spawn_enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
    }
}

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: EnemyType,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    position: Transform,
}

#[derive(Debug, Clone, Copy)]
pub enum ArcherStates {
    Walking,
    Attacking,
}

#[derive(Component, Clone, Copy)]
pub enum EnemyType {
    Basic,
    Archer(ArcherStates),
}

impl EnemyType {
    pub fn create_enemy(&self, position: Vec2, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<StandardMaterial>>) -> EnemyBundle {
        EnemyBundle {
            enemy: *self,
            mesh: Mesh3d(meshes.add(Cuboid::new(0.7,0.7,0.7))),
            material: MeshMaterial3d(materials.add(Color::srgb(1., 0., 0.))),
            position: Transform::from_xyz(position.x, 1., position.y),
        }
    }
}
