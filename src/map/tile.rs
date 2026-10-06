use bevy::prelude::*;

#[derive(Debug, Component, Clone, Copy, Default)]
pub enum TileType {
    #[default]
    Ground,
    Elevated(f32),
}

impl TileType {
    pub fn create_elevated_tile() -> TileType {
        Self::Elevated(0.5)
    }
}
