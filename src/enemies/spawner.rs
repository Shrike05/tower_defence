use std::fs;
use std::path::Path;
use std::time::Duration;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::enemies::basic::WalkNodes;
use crate::enemies::types::{EnemyInactive, EnemyType, create_enemy};
use crate::map::Map;
use crate::map::objectives::Spawner;

pub fn game_timer(
    mut enemies: Query<(Entity, &mut EnemyInactive)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (enemy, mut enemy_timer) in enemies.iter_mut() {
        enemy_timer.timer.tick(time.delta());
        if enemy_timer.timer.is_finished() {
            commands
                .entity(enemy)
                .remove::<EnemyInactive>()
                .insert(Visibility::Visible);
        }
    }
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let map = Map::from_map_file(Path::new("assets/levels/level0/level0.map"));
    let contents = fs::read_to_string(Path::new("assets/levels/level0/spawnjobs.enemies.toml"))
        .expect("Couldn't find file");
    let spawn_jobs: SpawnJobs = toml::from_str(&contents).expect("Couldn't parse file");

    let path = WalkNodes::from_path_file(Path::new("./assets/levels/level0/path0.path"), &map)
        .expect("Can't Find this WalkNodes instance");

    for spawn_job in spawn_jobs.jobs.iter() {
        for i in 0..spawn_job.enemies_count {
            commands.spawn_scene(create_enemy(
                EnemyType::Basic,
                path.clone(),
                Duration::from_secs_f32(
                    spawn_job.time_of_spawn + spawn_job.time_between_spawn * i as f32,
                ),
            ));
        }
    }
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
