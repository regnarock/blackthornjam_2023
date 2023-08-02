use bevy::prelude::*;

use crate::{mob::Mob, player::GamePlayer};

#[derive(Resource, Default)]
pub struct Target {
    pub entity: Option<Entity>,
}

pub fn update(
    mut commands: Commands,
    mut target: ResMut<Target>,
    mobs_query: Query<(&Transform, Entity), With<Mob>>,
    player_query: Query<&Transform, With<GamePlayer>>,
) {
    if target.entity.and_then(|e| commands.get_entity(e)).is_none() {
        if let Ok(player_pos) = player_query.get_single() {
            if let Some((_, e)) = mobs_query
                .iter()
                .min_by_key(|(t, _)| t.translation.distance(player_pos.translation) as i32)
            {
                target.entity = Some(e)
            }
        }
    }
}
