use bevy::prelude::*;

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

#[derive(Component, Debug, Copy, Clone, Default)]
pub struct EnemyProgress(pub f32);

impl EnemyType {
    pub fn create_enemy(
        &self,
        position: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<StandardMaterial>>,
    ) -> EnemyBundle {
        EnemyBundle {
            enemy: *self,
            mesh: Mesh3d(meshes.add(Cuboid::new(0.7, 0.7, 0.7))),
            material: MeshMaterial3d(materials.add(Color::srgb(1., 0., 0.))),
            position: Transform::from_xyz(position.x, 1., position.y),
            progress: EnemyProgress(0.),
        }
    }
}
