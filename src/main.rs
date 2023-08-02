// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod collisions;
pub mod game_over;
pub mod inputs;
pub mod mob;
pub mod player;
pub mod target;

use bevy::prelude::*;
use bevy_turborand::prelude::*;
use mob::MobsPlugin;
use target::Target;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    MainMenu,
    #[default]
    InGame,
    GameOverMenu,
}

const FIXED_TIMESTEP: f32 = 0.5;
fn main() {
    App::new()
        .init_resource::<Target>()
        .add_plugins(DefaultPlugins)
        .add_plugins(RngPlugin::default())
        .add_plugins(MobsPlugin)
        .add_state::<AppState>()
        .add_systems(OnEnter(AppState::InGame), setup)
        .add_systems(OnEnter(AppState::InGame), player::setup)
        .add_systems(
            Update,
            (
                collisions::check_player_and_mob_collision,
                inputs::process_keyboard_events,
                target::update,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnEnter(AppState::GameOverMenu), game_over::setup)
        //        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}
