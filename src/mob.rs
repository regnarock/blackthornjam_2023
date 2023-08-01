use bevy::{prelude::*, transform::commands, window::WindowCreated, text::{BreakLineOn, Text2dBounds}};
use bevy_turborand::prelude::*;

use crate::player::GamePlayer;

#[derive(Component)]
pub struct Mob;

#[derive(Component)]
pub struct Name {
    label: Vec<char>,
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let spawns_left = global_rng.bool();
    let spawns_up = global_rng.bool();

    let x = if spawns_left {
        32.0 - window.width() / 2.0
    } else {
        window.width() / 2.0 - 32.0
    };
    let font = asset_server.load("fonts/Chalkduster.ttf");
    let texture: Handle<Image> = asset_server.load("red_character.png");
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::new(x, 0.0, 0.0)),
            ..Default::default()
        },
        Mob,
        Name { label: "alfred".chars().collect() },
    )).with_children(|builder| {
        builder.spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    format!("alfred"),
                    TextStyle {
                        color: Color::RED,
                        font,
                        font_size: 42.0,
                    },
                )],
                alignment: TextAlignment::Left,
                linebreak_behavior: BreakLineOn::WordBoundary,
            },
            text_anchor: bevy::sprite::Anchor::TopCenter,
            // ensure the text is drawn on top of the box
            transform: Transform::from_translation(Vec3 { y: -17.0, x: 0.0, z: 1.0 }),
            ..default()
        });
    });
}

pub fn update_pos(
    mut commands: Commands,
    time: Res<Time>,
    mut mobs_query: Query<&mut Transform, (With<Mob>, Without<GamePlayer>)>,
    mut player_query: Query<&Transform, (With<GamePlayer>, Without<Mob>)>,
) {
    let player_transform = player_query.single_mut();
    for mut mob_transform in &mut mobs_query {
        // println!(
        //     "Player pos{}\nMob pos {}\nNormal projection: {}",
        //     player_transform.translation,
        //     mob_transform.translation,
        //     mob_transform.translation.project_onto(player_transform.translation).normalize_or_zero()
        // );
        let direction = player_transform.translation - mob_transform.translation;
        mob_transform.translation += direction / direction.length() * 100.0 * time.delta_seconds();
    }
}