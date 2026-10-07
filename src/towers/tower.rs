use std::{ops::Add, time::Duration};

use bevy::prelude::*;

use crate::{
    enemies::{EnemyHealth, EnemyInactive, EnemyType},
    map::{TileClickedMessage, TileSelection},
};

pub fn create_action(
    mut commands: Commands,
    tile_selection: Res<TileSelection>,
    mut tile_click_message: MessageReader<TileClickedMessage>,
) {
    for _ in tile_click_message.read() {
        if let Some((pos, tile)) = tile_selection.tile {
            let position = pos.translation.add(Vec3::Y);
            commands.spawn_scene(create_tower(
                Transform::from_translation(position),
                10.,
                5.,
                vec![IVec2::new(0, 0), IVec2::new(1, 0)],
            ));
        }
    }
}

pub fn tower_attack(
    mut towers_query: Query<(&Tower, &mut AttackTimer, &Transform)>,
    mut enemies_query: Query<(Entity, &Transform, &mut EnemyHealth), Without<EnemyInactive>>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for (tower, mut timer, transform) in towers_query.iter_mut() {
        timer.0.tick(time.delta());
        if !timer.0.is_finished() {
            continue;
        }
        timer.0.reset();

        let mut enemies_in_range: Vec<(Entity, &Transform, Mut<'_, EnemyHealth>)> =
            enemies_query.iter_mut().collect();
        if enemies_in_range.is_empty() {
            continue;
        }

        enemies_in_range[0].2.hp -= 1;

        if enemies_in_range[0].2.hp == 0 {
            commands.entity(enemies_in_range[0].0).despawn();
        }
    }
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
pub struct Tower {
    atk: f32,
    range: Vec<IVec2>,
}

#[derive(Component, Clone, Default, Debug)]
pub struct AttackTimer(Timer);
