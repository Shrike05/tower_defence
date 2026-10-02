use bevy::prelude::*;

use crate::enemies::basic::WalkNodes;

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: EnemyType,
    mesh: Mesh3d,
    material: MeshMaterial3d<StandardMaterial>,
    position: Transform,
    progress: EnemyProgress,
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

#[derive(Component, Debug, Clone)]
pub struct EnemyProgress {
    pub path: WalkNodes,
    pub progress: f32,
}

impl EnemyType {
    pub fn create_enemy(
        &self,
        path: WalkNodes,
        position: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> EnemyBundle {
        EnemyBundle {
            enemy: *self,
            mesh: Mesh3d(meshes.add(Cuboid::new(0.7, 0.7, 0.7))),
            material: MeshMaterial3d(materials.add(Color::srgb(1., 0., 0.))),
            position: Transform::from_xyz(position.x, 1., position.y),
            progress: EnemyProgress { path, progress: 0. },
        }
    }
}
