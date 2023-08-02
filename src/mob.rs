use bevy::{prelude::*, text::BreakLineOn};
use bevy_turborand::prelude::*;

use crate::{player::GamePlayer, AppState};

const MOB_SPEED: f32 = 100.0;

#[derive(Component)]
pub struct Mob {
    pub name: String,
    pub damages: u8,
}

impl Mob {
    pub fn new(name: String) -> Self {
        Mob { name, damages: 0 }
    }
}

#[derive(Resource)]
struct MobsConfig {
    mobs: Vec<MobConfig>,
    respawn_time_sec: f32,
    respawn_time_min_sec: f32,
}

pub struct MobConfig {
    pub name: String,
    pub asset_name: String,
}

pub struct TimeSinceLastSpawn {
    value: f32,
}

impl Default for TimeSinceLastSpawn {
    fn default() -> Self {
        Self { value: f32::MAX }
    }
}

fn spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut global_rng: ResMut<GlobalRng>,
    windows: Query<&Window>,
    time: Res<Time>,
    mut since_last_spawn: Local<TimeSinceLastSpawn>,
    mobs_config: Res<MobsConfig>,
) {
    (*since_last_spawn).value += time.delta_seconds();
    if since_last_spawn.value <= mobs_config.respawn_time_sec {
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
    let y = if !spawns_up {
        32.0 - window.height() / 2.0
    } else {
        window.height() / 2.0 - 32.0
    };

    let config = &mobs_config.mobs[global_rng.usize(0..mobs_config.mobs.len())];

    // TODO: don't reload the asset at each spawn, but load it once for good
    let font: Handle<Font> = asset_server.load("fonts/Chalkduster.ttf");
    let texture: Handle<Image> = asset_server.load(format!("{}.png", config.asset_name));

    commands
        .spawn((
            SpriteBundle {
                texture,
                transform: Transform::from_translation(Vec3::new(x, y, 0.0)),
                ..Default::default()
            },
            Mob::new(config.name.clone()),
        ))
        .with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        config.name.clone(),
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
                transform: Transform::from_translation(Vec3 {
                    y: -17.0,
                    x: 0.0,
                    z: 1.0,
                }),
                ..default()
            });
        });
}

fn update_pos(
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
        mob_transform.translation +=
            direction / direction.length() * MOB_SPEED * time.delta_seconds();
    }
}

fn check_dead(
    mut commands: Commands,
    mut mobs_query: Query<(&mut Mob, Entity)>,
    mut config: ResMut<MobsConfig>,
) {
    for (mob, entity) in &mut mobs_query {
        if mob.damages >= mob.name.len() as u8 {
            println!("Dead mob!");
            config.respawn_time_sec = (config.respawn_time_sec - 0.03).max(config.respawn_time_min_sec);
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub struct MobsPlugin;

impl Plugin for MobsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MobsConfig {
            respawn_time_sec: 5.0,
            respawn_time_min_sec: 0.5,
            mobs: vec![
                MobConfig {
                    name: "blue".to_string(),
                    asset_name: "purple_character".to_string(),
                },
                MobConfig {
                    name: "red".to_string(),
                    asset_name: "red_character".to_string(),
                },
                MobConfig {
                    name: "green".to_string(),
                    asset_name: "green_character".to_string(),
                },
            ],
        })
        .add_systems(
            Update,
            (spawn, update_pos, check_dead).run_if(in_state(AppState::InGame)),
        );
    }
}
