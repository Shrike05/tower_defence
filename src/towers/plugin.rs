use bevy::prelude::*;

use crate::towers::tower::{create_tower, setup};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}
