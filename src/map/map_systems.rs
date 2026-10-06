use super::tile::TileType;
use crate::map::objectives::{Objective, Spawner};
use bevy::prelude::*;
use std::fs;
use std::ops::Div;
use std::path::Path;

pub struct MapPlugin;

#[derive(Resource, Debug, Clone)]
pub struct Map {
    pub width: u32,
    tiles: Vec<TileType>,
}

#[derive(Resource, Clone, Debug, Default)]
pub struct TileSelection {
    pub tile: Option<(Transform, TileType)>,
}

#[derive(Message, Clone, Copy, Debug, Default)]
pub struct TileClickedMessage;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        let map = Map::from_map_file(Path::new("./assets/levels/level0/level0.map"));
        let spawner = Spawner::new(0, map.get_tile_world_coordinates(40));
        let objective = Objective::new(0, map.get_tile_world_coordinates(49));

        app.insert_resource(map);
        app.insert_resource(spawner);
        app.insert_resource(objective);
        app.insert_resource(TileSelection::default());
        app.add_message::<TileClickedMessage>();
        app.add_systems(Startup, setup);
    }
}

impl Map {
    pub fn from_map_file(path: &Path) -> Map {
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

    pub fn get_tile_coordinates(&self, i: usize) -> Vec2 {
        let width = self.width;
        let x = (i as u32).div(width) as f32;
        let z = (i as u32 % width) as f32;
        Vec2 { x, y: z }
    }

    pub fn get_tile_world_coordinates(&self, i: usize) -> Vec2 {
        let width = self.width;
        let height = (self.tiles.len() as u32).div(self.width);

        let x = 1.06 * ((i as u32).div(width) as f32 - height as f32 / 2.);
        let z = 1.06 * ((i as u32 % width) as f32 - width as f32 / 2.);

        Vec2 { x, y: z }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    map: Res<Map>,
) {
    let normal_mat = materials.add(Color::srgb(1., 1., 1.));
    let elevated_mat = materials.add(Color::srgb(0.7, 0.7, 0.7));
    let hover_mat = materials.add(Color::srgb(0.6, 0.6, 0.6));
    for (i, tile) in map.tiles.iter().enumerate() {
        let pos = map.get_tile_world_coordinates(i);
        let x = pos.x;
        let y = match tile {
            TileType::Ground => 0.,
            TileType::Elevated(y_value) => *y_value,
        };
        let z = pos.y;

        let mat = match tile {
            TileType::Ground => normal_mat.clone(),
            TileType::Elevated(_) => elevated_mat.clone(),
        };
        commands.spawn_scene(create_tile(x, y, z, *tile, mat, hover_mat.clone()));
    }
}

fn create_tile(
    x: f32,
    y: f32,
    z: f32,
    tile: TileType,
    normal_mat: Handle<StandardMaterial>,
    hover_mat: Handle<StandardMaterial>,
) -> impl Scene {
    let mat = normal_mat.clone();
    bsn! {
        Mesh3d(asset_value(Cuboid::default()))
        MeshMaterial3d<StandardMaterial>(mat)
        Transform::from_xyz(x, y, z)
        template_value(tile)

        on(move |event: On<Pointer<Over>>, mut query: Query<&mut MeshMaterial3d<StandardMaterial>>|{
            if let Ok(mut material) = query.get_mut(event.entity){
                material.0 = hover_mat.clone();
            }
        })

        on(move |event: On<Pointer<Out>>, mut query: Query<&mut MeshMaterial3d<StandardMaterial>>, mut tile_selection: ResMut<TileSelection>|{
            if let Ok(mut material) = query.get_mut(event.entity){
                material.0 = normal_mat.clone();
                tile_selection.tile = None;
            }
        })

        on(move |event: On<Pointer<Click>>, mut tile_selection: ResMut<TileSelection>, mut message_writer: MessageWriter<TileClickedMessage>|{
            tile_selection.tile = Some((Transform::from_xyz(x, y, z), tile));
            message_writer.write(TileClickedMessage);
        })
    }
}
