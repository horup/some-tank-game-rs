use bevy::prelude::*;

pub struct Hud {
    pub top_left_text:String,
    pub top_right_text:String,
    pub center_text:String
}

impl Default for Hud {
    fn default() -> Self {
        Self {
            top_left_text:"".into(),
            top_right_text:"".into(),
            center_text:"".into()
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum HudText {
    TopLeft,
    Center, 
    TopRight
}

fn hud_initialization_system(mut commands: Commands, asset_server: Res<AssetServer>, mut materials:ResMut<Assets<ColorMaterial>>) {
    let font_size = 16.0;

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
                "",
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
        }).insert(HudText::Center);
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
            "",
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
    }).insert(HudText::TopLeft);

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
            "",
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
    }).insert(HudText::TopRight);
}

pub fn set_text(text:&mut Text, value:&str) {
    let section = text.sections.first_mut().expect("atleast one section in hud text was expected!");
    if section.value != value {
        section.value = value.into();
    }
}

fn hud_system(hud:ResMut<Hud>, query:Query<(&mut Text, &HudText)>) {
    query.for_each_mut(|(mut text, hud_text)| {
        match *hud_text {
            HudText::TopLeft => {
                set_text(&mut text, &hud.top_left_text);
            }
            HudText::Center => {
                set_text(&mut text, &hud.center_text);
            }
            HudText::TopRight => {
                set_text(&mut text, &hud.top_right_text);
            }
        }
    });
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Hud::default());
        app.add_startup_system(hud_initialization_system.system());
        app.add_system(hud_system.system());
        
    }
}