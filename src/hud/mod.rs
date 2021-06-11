use bevy::{prelude::*};


#[derive(Clone, Copy)]
pub enum FadeDirection {
    In,
    Out
}
#[derive(Clone, Copy)]
pub struct FadeInOut {
    pub base_color:Color,
    pub elapsed_sec:f32,
    pub time_in_sec:f32,
    pub time_out_sec:f32,
    pub direction:FadeDirection
}

impl FadeInOut {
    pub fn new(mut base_color:Color, time_in_sec:f32, time_out_sec:f32) -> Self {
        FadeInOut {
            base_color,
            time_in_sec,
            time_out_sec,
            direction:FadeDirection::In,
            elapsed_sec:0.0
        }
    }
    pub fn alpha(&self) -> f32 {
        match self.direction {
            FadeDirection::In => self.elapsed_sec / self.time_in_sec,
            FadeDirection::Out => 1.0 - (self.elapsed_sec / self.time_out_sec),
        }
    }
}

pub struct Hud {
    pub top_left_text:String,
    pub top_right_text:String,
    pub center_text:String,
    pub foreground:Color,
    pub fade_in_out:Option<FadeInOut>
}

impl Hud {
    pub fn clear_texts(&mut self) {
        self.top_left_text = "".into();
        self.top_right_text = "".into();
        self.center_text = "".into();
    }

    pub fn fade(&mut self, time_in_sec:f32, time_out_sec:f32, base_color:Color) {
        if self.fade_in_out.is_none() {
            self.fade_in_out = Some(FadeInOut::new(base_color, time_in_sec, time_out_sec));
        }
    }
}

impl Default for Hud {
    fn default() -> Self {
        Self {
            top_left_text:"".into(),
            top_right_text:"".into(),
            center_text:"".into(),
            foreground:Color::rgba(1.0,1.0,1.0,0.0),
            fade_in_out:None
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum HudElement {
    TopLeft,
    Center, 
    TopRight,
    Foreground
}

fn hud_initialization_system(mut commands: Commands, asset_server: Res<AssetServer>, mut materials:ResMut<Assets<ColorMaterial>>) {
    let font_size = 16.0;

    commands.spawn_bundle(NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            //justify_content: JustifyContent::SpaceBetween,
            ..Default::default()
        },
        material: materials.add(Color::NONE.into()),
        ..Default::default()
    }).with_children(|parent| {

        // center text
        parent.spawn_bundle(NodeBundle {
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
                text: Text::with_section(
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
            }).insert(HudElement::Center);
        });

        // top-left text
        parent.spawn_bundle(TextBundle {
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
        }).insert(HudElement::TopLeft);

        // right text
        parent.spawn_bundle(TextBundle {
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
        }).insert(HudElement::TopRight);

        // foreground color ontop of all text
        parent.spawn_bundle(NodeBundle {
            style:Style {
                size:Size {
                    height:Val::Percent(100.0),
                    width:Val::Percent(100.0),
                },
                position_type: PositionType::Absolute,
                ..Default::default()
            },
            material:materials.add(Color::rgba(1.0, 0.0, 0.0, 0.5).into()),
            ..Default::default()
        }).insert(HudElement::Foreground);
    });
}

pub fn set_text(text:&mut Text, value:&str) {
    let section = text.sections.first_mut().expect("atleast one section in hud text was expected!");
    if section.value != value {
        section.value = value.into();
    }
}

fn update_text(hud:ResMut<Hud>, query:Query<(&mut Text, &HudElement)>) {
    if hud.is_changed() == false {
        return;
    }
    query.for_each_mut(|(mut text, element)| {
        match *element {
            HudElement::TopLeft => {
                set_text(&mut text, &hud.top_left_text);
            }
            HudElement::Center => {
                set_text(&mut text, &hud.center_text);
            }
            HudElement::TopRight => {
                set_text(&mut text, &hud.top_right_text);
            }
            _=>{}
        }
    });
}

fn update_foreground(hud:ResMut<Hud>, query:Query<(&mut Handle<ColorMaterial>, &HudElement, &Node)>, mut materials:ResMut<Assets<ColorMaterial>>) {
    query.for_each_mut(|(color_material_handle, element, _)| {
        if hud.is_changed() == false {
            return;
        }

        match *element {
            HudElement::Foreground => {
                if let Some(material) = materials.get_mut(color_material_handle.clone()) {
                    material.color = hud.foreground;
                }
            },
            _ => {}
        }
    });
}


fn fade_out_in_out(mut hud:ResMut<Hud>, time:Res<Time>) {
    if let Some(mut fade_in_out) = hud.fade_in_out {
        fade_in_out.elapsed_sec += time.delta_seconds();
        match fade_in_out.direction {
            FadeDirection::In => {
                if fade_in_out.elapsed_sec >= fade_in_out.time_in_sec {
                    fade_in_out.elapsed_sec = 0.0;
                    fade_in_out.direction = FadeDirection::Out;
                }

                hud.fade_in_out = Some(fade_in_out);

            },
            FadeDirection::Out => {
                if fade_in_out.elapsed_sec >= fade_in_out.time_out_sec {
                    fade_in_out.elapsed_sec = fade_in_out.time_out_sec;
                    hud.fade_in_out = None;
                } else {
                    hud.fade_in_out = Some(fade_in_out);
                }
            },
        }

        hud.foreground = fade_in_out.base_color;
        hud.foreground.set_a(fade_in_out.alpha());
    }
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Hud::default());
        app.add_startup_system(hud_initialization_system.system());
        app.add_system(update_text.system());
        app.add_system(fade_out_in_out.system().before("update_foreground"));
        app.add_system(update_foreground.system().label("update_foreground"));
        
    }
}