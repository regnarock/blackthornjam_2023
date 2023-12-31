use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{mob::Mob, player::GamePlayer, AppState};

pub fn check_player_and_mob_collision(
    mobs_query: Query<&Transform, (With<Mob>, Without<GamePlayer>)>,
    player_query: Query<&Transform, (With<GamePlayer>, Without<Mob>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let player_pos = player_query.single();

    for mob_transform in &mobs_query {
        let distance_to_player = player_pos
            .translation
            .xy()
            .distance(mob_transform.translation.xy());
        if distance_to_player <= 32.0 {
            next_state.set(AppState::GameOverMenu);
        }
    }
}
