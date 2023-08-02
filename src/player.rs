use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct GamePlayer;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("yellow_character.png"),
            ..Default::default()
        },
        GamePlayer,
    ));
}
