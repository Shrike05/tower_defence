use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::towers::tower::{create_action, create_tower};

pub struct TowerPlugin;

impl Plugin for TowerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, create_action);
    }
}
