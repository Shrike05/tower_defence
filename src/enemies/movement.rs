use crate::enemies::types::{EnemyInactive, EnemyProgress, EnemyType};
use bevy::prelude::*;

//Compute only partially over several frames
pub fn update_enemy_positions(
    mut enemies_query: Query<
        (&mut Transform, &mut EnemyProgress),
        (With<EnemyType>, Without<EnemyInactive>),
    >,
    time: Res<Time>,
) {
    enemies_query
        .par_iter_mut()
        .for_each(|(mut pos, mut progress)| {
            progress.progress += 0.1 * time.delta_secs();
            if progress.progress >= 1. {
                progress.progress = 1.;
            }
            let new_pos = progress.path.get_pos(progress.progress);
            pos.translation = Vec3::new(new_pos.x, 1., new_pos.y);
        });
}
