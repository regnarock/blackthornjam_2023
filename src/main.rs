// Bevy code commonly triggers these lints and they may be important signals
// about code quality. They are sometimes hard to avoid though, and the CI
// workflow treats them as errors, so this allows them throughout the project.
// Feel free to delete this line.
#![allow(clippy::too_many_arguments, clippy::type_complexity)]

pub mod player;
pub mod mob;

use bevy::prelude::*;
use bevy_turborand::prelude::*;

const FIXED_TIMESTEP: f32 = 0.5;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RngPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Startup, player::setup)
        .add_systems(Startup, mob::spawn)
        .add_systems(Update, mob::update_pos)
//        .insert_resource(FixedTime::new_from_secs(FIXED_TIMESTEP))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
}
