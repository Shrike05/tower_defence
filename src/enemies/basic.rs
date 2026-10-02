use bevy::{platform::collections::HashMap, prelude::*};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

use crate::map::Map;
const SEARCH_DEPTH: u32 = 100;

#[derive(Debug, Clone, PartialEq)]
pub struct WalkNodes {
    path: Vec<Vec2>,
}

impl WalkNodes {
    pub fn from_path_file(file: &Path, map: &Map) -> Option<Self> {
        let pairs = parse_file(file)?;

        let path = pairs
            .iter()
            .map(|pair| {
                let p = pair.1;
                let i = p.y + p.x * map.width as i32;
                map.get_tile_world_coordinates(i as usize)
            })
            .collect();

        Some(WalkNodes { path })
    }
    pub fn shortest_path(
        start: IVec2,
        end: IVec2,
        filter_map: HashMap<IVec2, bool>,
    ) -> Option<Self> {
        let path = a_star(&start, &end, filter_map)?;
        Some(WalkNodes {
            path: path.iter().map(|p| p.as_vec2()).collect(),
        })
    }

    pub fn path_len(&self) -> f32 {
        self.path.len() as f32
    }

    pub fn get_pos(&self, progress: &f32) -> Vec2 {
        let pure_progress = progress * self.path_len() - 1.;
        let fractional = 1. - (1. - pure_progress.fract()).powi(3);
        let segment = pure_progress.floor() as usize;

        let b = self.path[segment];
        let a = self.path[if pure_progress >= self.path_len() - 1. {
            segment
        } else {
            segment + 1
        }];

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

fn parse_file(file: &Path) -> Option<Vec<(IVec2, IVec2)>> {
    let file = File::open(file).ok()?;
    let reader = BufReader::new(file);
    let mut pairs = Vec::new();

    for line in reader.lines() {
        let line = line.ok()?;
        let line = line.trim();

        // Skip empty lines
        if line.is_empty() {
            continue;
        }

        // Split the line by the arrow "->"
        let parts: Vec<&str> = line.split("->").collect();
        if parts.len() != 2 {
            continue; // Skip lines that don't match the expected structure
        }

        // Helper closure to parse strings like "( 4, 74 )" into [i32; 2]
        let parse_coord = |s: &str| -> Option<IVec2> {
            let s = s.trim();
            let s = s.strip_prefix('(')?.strip_suffix(')')?;
            let coords: Vec<&str> = s.split(',').collect();
            if coords.len() != 2 {
                return None;
            }
            let x = coords[0].trim().parse::<i32>().ok()?;
            let y = coords[1].trim().parse::<i32>().ok()?;
            Some(IVec2::new(x, y))
        };

        if let (Some(c1), Some(c2)) = (parse_coord(parts[0]), parse_coord(parts[1])) {
            pairs.push((c1, c2));
        }
    }

    Some(pairs)
}
