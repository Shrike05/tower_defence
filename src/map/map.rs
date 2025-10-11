use super::tile::TileType;
use crate::map::objectives::{Objective, Spawner};
use crate::map::tile::TilePlugin;
use bevy::prelude::*;
use std::fs;
use std::ops::Div;
use std::path::Path;

pub struct MapPlugin;

#[derive(Resource)]
struct Map {
    width: u32,
    tiles: Vec<TileType>,
}

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let map = Map::from_map_file(Path::new("./level0.map"));
        let spawner = Spawner::new(0, map.get_tile_world_coordinates(40));
        let objective = Objective::new(0, map.get_tile_world_coordinates(49));

        app.insert_resource(map);
        app.insert_resource(spawner);
        app.insert_resource(objective);
        app.add_plugins(TilePlugin);
        app.add_systems(Startup, setup);
    }
}

impl Map {
    fn from_map_file(path: &Path) -> Map {
        let mut result: Vec<TileType> = vec![];

        let content = fs::read_to_string(path).expect(&format!("Couldn't find {:?}", path));

        for i in content.chars() {
            match i {
                'O' => result.push(TileType::create_elevated_tile()),
                '#' => result.push(TileType::Ground),
                _ => (),
            }
        }

        Map {
            width: content.find('\n').unwrap() as u32,
            tiles: result,
        }
    }

    fn get_tile_coordinates(&self, i: usize) -> Vec2 {
        let width = self.width;
        let x = (i as u32).div(width) as f32;
        let z = (i as u32 % width) as f32;
        Vec2 { x: x, y: z }
    }

    fn get_tile_world_coordinates(&self, i: usize) -> Vec2 {
        let width = self.width;
        let height = (self.tiles.len() as u32).div(self.width);

        let x = 1.06 * ((i as u32).div(width) as f32 - height as f32 / 2.);
        let z = 1.06 * ((i as u32 % width) as f32 - width as f32 / 2.);

        Vec2 { x: x, y: z }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<Map>,
) {
    for (i, tile) in map.tiles.iter().enumerate() {
        let pos = map.get_tile_world_coordinates(i);
        let x = pos.x;
        let z = pos.y;
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(Color::srgb(1., 1., 1.))),
            Transform::from_xyz(x as f32, 0., z as f32),
            *tile,
        ));
    }
}