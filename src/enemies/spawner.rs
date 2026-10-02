use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::enemies::basic::WalkNodes;
use crate::enemies::types::{EnemyType, create_enemy};
use crate::map::Map;
use crate::map::objectives::Spawner;

#[derive(Resource, Debug, Clone)]
pub struct GameTimer(pub Timer);

pub fn run_spawn_jobs(
    mut timer: ResMut<GameTimer>,
    spawn_jobs_handle: Res<SpawnConfigurationHandle>,
    spawn_asset: Res<Assets<SpawnJobs>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    timer.0.tick(time.delta());
    if let Some(spawn_jobs) = spawn_asset.get(&spawn_jobs_handle.handle) {
        for spawn_job in spawn_jobs.jobs.iter() {
            if (spawn_job.time_of_spawn - timer.0.elapsed_secs()).abs() < 0.1 {
                println!("Spawning");
                let path = WalkNodes::from_path_file(
                    Path::new("./assets/levels/level0/path0.path"),
                    &spawn_jobs_handle.map,
                )
                .expect("Can't Find this WalkNodes instance");
                commands.spawn_scene(create_enemy(EnemyType::Basic, path));
            }
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("levels/level0/spawnjobs.enemies.toml");
    commands.insert_resource(SpawnConfigurationHandle {
        map: Map::from_map_file(Path::new("./assets/levels/level0/level0.map")),
        handle,
    });
}

#[derive(Resource, Clone, Debug)]
pub struct SpawnConfigurationHandle {
    map: Map,
    handle: Handle<SpawnJobs>,
}

#[derive(Serialize, Deserialize, Asset, TypePath, Debug, Clone)]
pub struct SpawnJobs {
    pub jobs: Vec<SpawnJob>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpawnJob {
    pub time_of_spawn: f32,
    pub time_between_spawn: f32,
    pub enemies_count: u32,
    pub enemy_type: EnemyType,
    pub path: String,
}
