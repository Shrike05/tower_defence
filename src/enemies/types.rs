use std::time::Duration;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::enemies::basic::WalkNodes;

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct EnemyHealth {
    max_hp: u32,
    pub hp: u32,
}

#[derive(Component, Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub enum EnemyType {
    #[default]
    Basic,
    Archer,
}

#[derive(Component, Debug, Clone, Default)]
pub struct EnemyProgress {
    pub path: WalkNodes,
    pub progress: f32,
}

#[derive(Component, Debug, Clone, Default)]
pub struct EnemyInactive {
    pub timer: Timer,
}

pub fn create_enemy(
    enemy_type: EnemyType,
    path: WalkNodes,
    inactivity_time: Duration,
) -> impl Scene {
    let hp_component = match enemy_type {
        EnemyType::Basic => EnemyHealth { max_hp: 10, hp: 10 },
        EnemyType::Archer => EnemyHealth { max_hp: 7, hp: 7 },
    };
    bsn! {
        Mesh3d(asset_value(Cuboid::new(0.7, 0.7, 0.7)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(1., 0., 0.)))
        Transform::from_xyz(path.get_pos(0.).x, 1., path.get_pos(0.).y)
        EnemyProgress { path, progress: 0. }
        template_value(enemy_type)
        EnemyInactive{timer: Timer::new(inactivity_time, TimerMode::Once)}
        Visibility::Hidden
        template_value(hp_component)
    }
}
