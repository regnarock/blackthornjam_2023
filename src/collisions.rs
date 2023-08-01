use bevy::{prelude::*, math::Vec3Swizzles};

use crate::{mob::Mob, player::GamePlayer, AppState};

pub fn check_player_and_mob_collision(
    mobs_query: Query<&Transform, (With<Mob>, Without<GamePlayer>)>,
    player_query: Query<(&Transform, Entity), (With<GamePlayer>, Without<Mob>)>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let (player_pos, player_e) = player_query.single();

    for mob_transform in &mobs_query {
        if player_pos.translation.xy().distance(mob_transform.translation.xy()) <= 32.0 {
            next_state.set(AppState::GameOverMenu);
        }
    }
}