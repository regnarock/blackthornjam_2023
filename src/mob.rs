use bevy::{prelude::*, text::BreakLineOn};
use bevy_turborand::prelude::*;

use crate::player::GamePlayer;

const MOB_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Mob {
    pub name: String,
    pub damages: u8,
}

impl Mob {
    pub fn new(name: String) -> Self {
        Mob {
            name,
            damages: 0,
        }
    }
}

pub struct TimeSinceLastSpawn {
    value: f32,
}

impl Default for TimeSinceLastSpawn {
    fn default() -> Self {
        Self { value: f32::MAX }
    }
}

pub fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
    windows: Query<&Window>,
    time: Res<Time>,
    mut since_last_spawn: Local<TimeSinceLastSpawn>
) {
    (*since_last_spawn).value += time.delta_seconds();
    if since_last_spawn.value <= 2.0 {
        return;
    }
    (*since_last_spawn).value = 0.0;
    let window = windows.single();
    let spawns_left = global_rng.bool();
    let spawns_up = global_rng.bool();

    let x = if spawns_left {
        32.0 - window.width() / 2.0
    } else {
        window.width() / 2.0 - 32.0
    };
    let font: Handle<Font> = asset_server.load("fonts/Chalkduster.ttf");
    let texture: Handle<Image> = asset_server.load("red_character.png");
    let mob_name = "killme".to_string();
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_translation(Vec3::new(x, 0.0, 0.0)),
            ..Default::default()
        },
        Mob::new(mob_name.clone()),
    )).with_children(|builder| {
        builder.spawn(Text2dBundle {
            text: Text {
                sections: vec![TextSection::new(
                    mob_name,
                    TextStyle {
                        color: Color::RED,
                        font,
                        font_size: 42.0,
                    },
                )],
                alignment: TextAlignment::Center,
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
        mob_transform.translation += direction / direction.length() * MOB_SPEED * time.delta_seconds();
    }
}

pub fn check_dead(
    mut commands: Commands,
    mut mobs_query: Query<(&mut Mob, Entity)>,
) {
    for (mob, entity) in &mut mobs_query {
        if mob.damages >= mob.name.len() as u8 {
            println!("Dead mob!");
            commands.entity(entity).despawn_recursive();
        }
    }
}