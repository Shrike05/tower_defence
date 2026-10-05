use std::time::Duration;

use bevy::prelude::*;

pub fn setup(mut commands: Commands) {
    commands.spawn_scene(create_tower(
        Transform::from_xyz(2., 2., 2.),
        1.,
        5.,
        vec![IVec2::new(0, 0), IVec2::new(1, 0)],
    ));
}

pub fn create_tower(
    transform: Transform,
    atk: f32,
    atk_rate: f32,
    range: Vec<IVec2>,
) -> impl Scene {
    bsn! {
        template_value(transform)
        Mesh3d(asset_value(Cuboid::new(0.7, 1., 0.7)))
        MeshMaterial3d<StandardMaterial>(asset_value(Color::srgb(0., 0., 1.)))
        Tower {atk, range}
        AttackTimer(Timer::new(Duration::from_secs_f32(1./atk_rate), TimerMode::Repeating))
    }
}

#[derive(Component, Debug, Clone, Default)]
struct Tower {
    atk: f32,
    range: Vec<IVec2>,
}

#[derive(Component, Clone, Default, Debug)]
struct AttackTimer(Timer);
