use bevy::{core::FixedTimestep, prelude::*};

use crate::RootNode;

struct FPSText;
pub struct DiagnosticsPlugin;

fn startup(mut commands:Commands, asset_server: Res<AssetServer>, mut materials:ResMut<Assets<ColorMaterial>>, root_node:Query<(Entity, &RootNode)>) {
    let root_node = root_node.single().expect("root node not found").0;
    commands.entity(root_node).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            style:Style {
                position_type:PositionType::Absolute,
                position:Rect {
                    top:Val::Px(16.0),
                    right:Val::Px(16.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text:Text::with_section("Hello world", TextStyle {
                font: asset_server.load("fonts/default.ttf"),
                color: Color::RED,
                font_size:16.0,
                ..Default::default()
            }, 
            TextAlignment {
                ..Default::default()
            }),
            ..Default::default()
        }).insert(FPSText);
    });
}

fn update(query:Query<(&mut Text, &FPSText)>, time:Res<Time>) {
    query.for_each_mut(|(mut text, _)| {
        if let Some(section) = text.sections.get_mut(0) {
            section.value = format!("{}", (1.0 / time.delta_seconds()) as u32);
        }
    });
}

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update.system().with_run_criteria(FixedTimestep::steps_per_second(1.0)));
        app.add_startup_system(startup.system());
    }
}