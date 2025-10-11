use bevy::prelude::*;

pub struct TilePlugin;

impl Plugin for TilePlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, setup_elevated_tile);
    }
}


#[derive(Debug, Component, Clone, Copy)]
pub enum TileType{
    Ground,
    Elevated(f32)
}

impl TileType{
    pub fn create_elevated_tile() -> TileType{
        Self::Elevated(0.5)
    }
}

fn setup_elevated_tile( 
    mut new_tiles_query: Query<(&TileType, &mut Transform, &mut MeshMaterial3d<StandardMaterial>), Added<TileType>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
){
    for (tile_type, mut transform, mut material) in new_tiles_query.iter_mut(){
        if let TileType::Elevated(height) = tile_type {
            transform.translation.y += height;
            material.0 = materials.add(Color::srgb(0.7, 0.7, 0.7))
        }
    } 
}