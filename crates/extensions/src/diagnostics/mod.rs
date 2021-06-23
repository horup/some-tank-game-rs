use bevy::prelude::*;

struct FPS;
pub struct DiagnosticsPlugin;

fn startup(mut commands:Commands, asset_server: Res<AssetServer>, mut materials:ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            position_type:PositionType::Absolute,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    }).with_children(|parent| {
        parent.spawn_bundle(TextBundle {
            style:Style {
                position:Rect {
                    ..Default::default()
                },
                ..Default::default()
            },
            text:Text::with_section("Hello world", TextStyle {
                font: asset_server.load("fonts/default.ttf"),
                color: Color::RED,
                ..Default::default()
            }, 
            TextAlignment {
                ..Default::default()
            }),
            ..Default::default()
        }).insert(FPS);
    });
}

fn update() {

}

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(update.system());
        app.add_startup_system(startup.system());
    }
}