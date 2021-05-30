use bevy::prelude::*;

pub fn ui_initialization_system(mut commands: Commands, asset_server: Res<AssetServer>, mut materials:ResMut<Assets<ColorMaterial>>) {
    // UI camera
    commands.spawn_bundle(UiCameraBundle::default());

    let font_size = 16.0;

    // center text
  /*  commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            align_items: AlignItems::FlexEnd,
            justify_content:JustifyContent::Center,
            size:Size {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Center\nText",
            TextStyle {
                font: asset_server.load("fonts/default.ttf"),
                font_size: font_size * 2.0,
                color: Color::WHITE,
            },

            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
                ..Default::default()
            },
        ),
        ..Default::default()
    });*/

    commands.spawn_bundle(NodeBundle {
        style:Style {
            size:Size {
                height:Val::Percent(100.0),
                width:Val::Percent(100.0),
            },
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
            ..Default::default()
        },
        material:materials.add((Color::NONE).into()),
        ..Default::default()
    })
    .with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::Center,
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            // Use the `Text::with_section` constructor
            text: Text::with_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "Center\nText",
                TextStyle {
                    font: asset_server.load("fonts/default.ttf"),
                    font_size:font_size * 2.0,
                    color: Color::WHITE,
                },
                // Note: You can use `Default::default()` in place of the `TextAlignment`
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    vertical: VerticalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        });
    });

    // top-left text
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.0),
                left: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Top\nLeft\nText",
            TextStyle {
                font: asset_server.load("fonts/default.ttf"),
                font_size,
                color: Color::WHITE,
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Left,
                ..Default::default()
            },
        ),
        ..Default::default()
    });

    // right text
    commands.spawn_bundle(TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(0.0),
                right: Val::Px(0.0),
                ..Default::default()
            },
            ..Default::default()
        },
        // Use the `Text::with_section` constructor
        text: Text::with_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Top\nRight\nText",
            TextStyle {
                font: asset_server.load("fonts/default.ttf"),
                font_size,
                color: Color::WHITE,
            },
            // Note: You can use `Default::default()` in place of the `TextAlignment`
            TextAlignment {
                horizontal: HorizontalAlign::Right,
                ..Default::default()
            },
        ),
        ..Default::default()
    });
}
