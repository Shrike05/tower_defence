use bevy::prelude::*;
use derive_new::new;

#[derive(Resource, new)]
pub struct Spawner{
    id: u32,
    pub position: Vec2
}
#[derive(Resource, new)]
pub struct Objective{
    id: u32,
    position: Vec2
}