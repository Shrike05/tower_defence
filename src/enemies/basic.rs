use crate::map::Map;
use bevy::{platform::collections::HashMap, prelude::*};
use std::path::Path;
const SEARCH_DEPTH: u32 = 100;

#[derive(Debug, Clone, PartialEq, Component)]
pub struct WalkNodes {
    path: Vec<IVec2>,
}

#[derive(Component, Clone, Debug, PartialEq)]
pub struct EnemyPos {
    progress: Vec<f32>,
}

impl EnemyPos {
    pub fn add(&mut self, next_pos: f32) {
        self.progress.push(next_pos);
    }

    pub fn set(&mut self, path: Vec<f32>) {
        self.progress = path;
    }

    pub fn step(&mut self) -> f32 {
        self.progress.remove(0)
    }
}

impl WalkNodes {
    pub fn from_path_file(file: &Path, map: &Map) -> Option<Self> {
        None
    }
    pub fn shortest_path(
        start: IVec2,
        end: IVec2,
        filter_map: HashMap<IVec2, bool>,
    ) -> Option<Self> {
        let path = a_star(&start, &end, filter_map)?;
        Some(WalkNodes { path })
    }

    pub fn path_len(&self) -> f32 {
        self.path.len() as f32
    }

    pub fn get_pos(&self, progress: &f32) -> Vec2 {
        let pure_progress = progress * self.path_len() - 1.;
        let fractional = pure_progress.fract();
        let segment = pure_progress.floor() as usize;

        let a = self.path[segment].as_vec2();
        let b = self.path[segment].as_vec2();

        fractional * a + (1. - fractional) * b
    }
}

fn a_star(start: &IVec2, end: &IVec2, filter_map: HashMap<IVec2, bool>) -> Option<Vec<IVec2>> {
    filter_map.get(end)?;

    let h = |x: IVec2| (end.x - x.x).abs() + (end.y - x.y).abs();
    let mut next_to_search: Vec<IVec2> = vec![*start];
    let mut f_scores: HashMap<IVec2, i32> = HashMap::new();

    let mut came_from: HashMap<IVec2, IVec2> = HashMap::new();

    let mut g_scores: HashMap<IVec2, i32> = HashMap::new();
    g_scores.insert(*start, 0);

    let mut current_depth = 0_u32;
    while !next_to_search.is_empty() && current_depth < SEARCH_DEPTH {
        current_depth += 1;

        let current = next_to_search.remove(0);
        if current == *end {
            let mut total_path = vec![current];
            let mut backward = current;
            while came_from.contains_key(&backward) {
                backward = came_from[&backward];
                total_path.push(backward);
            }
            return Some(total_path);
        }

        let neighbours: Vec<IVec2> = [
            current + IVec2::X,
            current + IVec2::Y,
            current - IVec2::X,
            current - IVec2::Y,
        ]
        .iter()
        .filter_map(|neighbour| {
            if neighbour.x < 0 || neighbour.y < 0 || filter_map.get(neighbour).is_some() {
                None
            } else {
                Some(*neighbour)
            }
        })
        .collect();

        for neighbour in neighbours {
            let this_g_score = g_scores[&current] + 1;
            let g_score = g_scores.get(&neighbour).unwrap_or(&i32::MAX);
            let already_exists = g_scores.contains_key(&neighbour);

            if this_g_score < *g_score {
                g_scores.insert(neighbour, this_g_score);
                came_from.insert(neighbour, current);
                f_scores.insert(neighbour, this_g_score + h(neighbour));
                if !already_exists {
                    next_to_search.push(neighbour);
                }
            }
        }
    }

    None
}
