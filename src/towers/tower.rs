use std::{ops::Add, time::Duration};

use bevy::prelude::*;

use crate::map::{TileClickedMessage, TileSelection};

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
