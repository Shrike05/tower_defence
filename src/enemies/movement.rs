use crate::enemies::basic::*;
use bevy::prelude::*;
use rayon::prelude::*;

pub fn movement(enemies_query: Query<(&mut EnemyPos, &WalkNodes, &mut Transform)>) {
    for (mut ePos, nodes, transform) in enemies_query {
        let pos = nodes.get_pos(&ePos.step());
    }
}

//Compute only partially over several frames
pub fn compute_progress(mut enemies_query: Query<(&mut EnemyPos, &WalkNodes)>) {
    let speed = 0.1;
    enemies_query.par_iter_mut().for_each(|(mut pos, nodes)| {
        let path_len = nodes.path_len();
        let my_speed = speed / path_len;

        let paths = (0..path_len as u32)
            .into_par_iter()
            .map(|part| my_speed * part as f32)
            .collect::<Vec<f32>>();

        pos.set(paths);
    });
}
