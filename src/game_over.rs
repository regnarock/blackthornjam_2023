use bevy::{prelude::*, text::BreakLineOn};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/Chalkduster.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                // fill the entire window
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: BackgroundColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {
            builder.spawn(TextBundle::from_section(
                "Game Over!",
                TextStyle {
                    font,
                    font_size: 140.0,
                    color: Color::RED,
                },
            ));
        });
}
