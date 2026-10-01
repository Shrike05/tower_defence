use bevy::prelude::*;
use std::f32::consts::PI;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (camera.spawn(), directional_light.spawn()));
    }
}

fn camera() -> impl Scene {
    let transform = Transform::from_xyz(-4., 10., 0.).looking_at(Vec3::ZERO, Vec3::Y);
    bsn! {
        Camera3d::default()
        template_value(transform)
    }
}

fn directional_light() -> impl Scene {
    bsn! {
        DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
        }
        Transform {
            translation: Vec3::new(0.0, 1.5, 0.0),
            rotation: Quat::from_rotation_x(-PI / 6.),
        }
    }
}
