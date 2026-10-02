use crate::enemies::{
    basic::*,
    types::{EnemyProgress, EnemyType},
};
use bevy::prelude::*;

//Compute only partially over several frames
pub fn update_enemy_positions(
    mut enemies_query: Query<(&mut Transform, &EnemyProgress), With<EnemyType>>,
    walknodes: Res<WalkNodes>,
) {
    enemies_query
        .par_iter_mut()
        .for_each(|(mut pos, progress)| {
            let new_pos = walknodes.get_pos(&progress.0);
            pos.translation = Vec3::new(new_pos.x, 1., new_pos.y);
        });
}

pub fn update_enemy_progress(mut enemy_query: Query<&mut EnemyProgress>, time: Res<Time>) {
    enemy_query.iter_mut().for_each(|mut progress| {
        progress.0 += 0.1 * time.delta_secs();
        if progress.0 >= 1. {
            progress.0 = 1.;
        }
    });
}
