use bevy::{core::FixedTimestep, prelude::*};
use bevy_egui::{EguiContext, egui::{self, Align, Color32, Direction, Label, Layout}};
use extensions::RootNode;

use crate::{Config, Console};

#[derive(Clone, Copy, Debug)]
pub enum FadeDirection {
    In,
    Out,
    InBetween,
}
#[derive(Clone, Copy)]
pub struct FadeInOut {
    pub base_color: Color,
    pub elapsed_sec: f32,
    pub time_in_sec: f32,
    pub time_out_sec: f32,
    pub time_in_between_sec: f32,
    pub direction: FadeDirection,
}

impl FadeInOut {
    pub fn new(
        base_color: Color,
        time_in_sec: f32,
        time_out_sec: f32,
        time_between_sec: f32,
    ) -> Self {
        FadeInOut {
            base_color,
            time_in_sec,
            time_out_sec,
            time_in_between_sec: time_between_sec,
            direction: FadeDirection::In,
            elapsed_sec: 0.0,
        }
    }
    pub fn alpha(&self) -> f32 {
        match self.direction {
            FadeDirection::In => self.elapsed_sec / self.time_in_sec,
            FadeDirection::Out => 1.0 - (self.elapsed_sec / self.time_out_sec),
            FadeDirection::InBetween => 1.0,
        }
    }
}

#[derive(Clone)]
pub struct Hud {
    pub console_text: String,
    pub top_left_text: String,
    pub top_right_text: String,
    pub bottom_center_text: String,
    pub bottom_left_text: String,
    pub center_text: String,
    pub foreground: Color,
    pub background: Color,
    pub fade_in_out: Option<FadeInOut>,
    pub show_console: bool,
    pub stack:Vec<Hud>
}


impl Hud {
    pub fn clear_texts(&mut self) {
        self.top_left_text = "".into();
        self.top_right_text = "".into();
        self.center_text = "".into();
        self.bottom_center_text = "".into();
        self.bottom_left_text = "".into();
    }

    pub fn clear(&mut self) {
        let stack = self.stack.clone();
        *self = Hud::default();
        self.stack = stack;
    }

    pub fn push(&mut self) {
        let mut clone = self.clone();
        clone.console_text.clear();
        clone.stack.clear();
        self.stack.push(clone);
    }

    pub fn pop(&mut self) {
        let mut stack = self.stack.clone();
        let console = self.console_text.clone();
        if let Some(mut pop) = stack.pop() {
            pop.stack = stack;
            *self = pop;
            self.console_text = console;
        }
    }

    pub fn fade(&mut self, time_in_sec: f32, time_out_sec: f32, base_color: Color) {
        if self.fade_in_out.is_none() {
            self.fade_in_out = Some(FadeInOut::new(
                base_color,
                time_in_sec,
                time_in_sec / 2.0 + time_out_sec / 2.0,
                0.5,
            ));
        }
    }

    pub fn start_default_fade(&mut self) {
        self.fade(0.5, 0.5, Color::BLACK);
    }
}

impl Default for Hud {
    fn default() -> Self {
        Self {
            console_text: "".into(),
            bottom_left_text: "".into(),
            top_left_text: "".into(),
            top_right_text: "".into(),
            center_text: "".into(),
            bottom_center_text: "".into(),
            background: Color::rgba(1.0, 1.0, 1.0, 0.0),
            foreground: Color::rgba(1.0, 1.0, 1.0, 0.0),
            fade_in_out: None,
            show_console: false,
            stack:Vec::new()
        }
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum HudElement {
    TopLeft,
    Center,
    TopRight,
    Foreground,
    Background,
    BottomCenter,
    Console,
    BottomLeft
}

pub struct FPSText;

fn hud_initialization_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    root_node: Query<(Entity, &RootNode)>,
    config: Res<Config>,
    windows: Res<Windows>
) {
    let scale_factor = windows.get_primary()
    .and_then(|x| Some(x.scale_factor()))
    .unwrap_or(1.0);
    
    let font_size = 16.0 / scale_factor as f32;
    let root_node = root_node.single().expect("root node not found").0;
    
    commands.entity(root_node).with_children(|parent| {
        // center text
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    ..Default::default()
                },
                material: materials.add((Color::NONE).into()),
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            position_type: PositionType::Absolute,
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/default.ttf"),
                                font_size: font_size * 3.0,
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
                    })
                    .insert(HudElement::Center);

                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::Center,
                            position_type: PositionType::Absolute,
                            position: Rect {
                                bottom: Val::Percent(25.0),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text::with_section(
                            "",
                            TextStyle {
                                font: asset_server.load("fonts/default.ttf"),
                                font_size: font_size * 2.0,
                                color: Color::WHITE,
                            },
                            // Note: You can use `Default::default()` in place of the `TextAlignment`
                            TextAlignment {
                                horizontal: HorizontalAlign::Center,
                                vertical: VerticalAlign::Bottom,
                                ..Default::default()
                            },
                        ),
                        ..Default::default()
                    })
                    .insert(HudElement::BottomCenter);
            });

        // top-left text
        parent
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(16.0),
                        left: Val::Px(16.0),
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
            })
            .insert(HudElement::TopLeft);

        // bottom-left text
        parent
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(16.0),
                    left: Val::Px(16.0),
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
        })
        .insert(HudElement::BottomLeft);

        // right text
        parent
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(16.0),
                        right: Val::Px(16.0),
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
            })
            .insert(HudElement::TopRight);

        // foreground color ontop of all, except for console and fps
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    size: Size {
                        height: Val::Percent(100.0),
                        width: Val::Percent(100.0),
                    },
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                material: materials.add(Color::rgba(1.0, 0.0, 0.0, 0.5).into()),
                ..Default::default()
            })
            .insert(HudElement::Foreground);

        // console
        parent
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size {
                        height: Val::Percent(50.0),
                        width: Val::Percent(100.0),
                    },
                    position: Rect {
                        left: Val::Px(0.0),
                        right: Val::Percent(100.0),
                        top: Val::Px(0.0),
                        bottom: Val::Percent(50.0),
                    },
                    ..Default::default()
                },
                material: materials.add((Color::rgba(0.0, 0.0, 0.0, 0.95)).into()),
                visible: Visible {
                    is_visible: false,
                    is_transparent: true,
                },
                ..Default::default()
            })
            .with_children(|parent| {
                parent
                    .spawn_bundle(TextBundle {
                        style: Style {
                            align_self: AlignSelf::FlexEnd,
                            position_type: PositionType::Absolute,
                            position: Rect {
                                left: Val::Px(8.0),
                                bottom: Val::Px(8.0),
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
                    })
                    .insert(HudElement::Console);
            })
            .insert(HudElement::Console);

            
        if config.show_fps() {
            // fps text
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(64.0), Val::Px(32.0)),
                        position: Rect {
                            right: Val::Px(0.0),
                            top: Val::Px(0.0),
                            ..Default::default()
                        },
                        position_type: PositionType::Absolute,
                        ..Default::default()
                    },
                    visible:Visible {
                        is_visible: true,
                        is_transparent: true,
                    },
                    material: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.75).into()),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: Rect {
                                    top: Val::Px(8.0),
                                    right: Val::Px(8.0),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            text: Text::with_section(
                                "0",
                                TextStyle {
                                    font: asset_server.load("fonts/default.ttf"),
                                    color: Color::RED,
                                    font_size: 16.0,
                                    ..Default::default()
                                },
                                TextAlignment {
                                    ..Default::default()
                                },
                            ),
                            ..Default::default()
                        })
                        .insert(FPSText);
                });
        }
    }).insert(HudElement::Background);
}

pub fn set_text(text: &mut Text, value: &str) {
    let section = text
        .sections
        .first_mut()
        .expect("atleast one section in hud text was expected!");
    if section.value != value {
        section.value = value.into();
    }
}

fn update_text(hud: ResMut<Hud>, query: Query<(&mut Text, &HudElement)>) {
    if hud.is_changed() == false {
        return;
    }
    query.for_each_mut(|(mut text, element)| match *element {
        HudElement::TopLeft => {
            set_text(&mut text, &hud.top_left_text);
        }
        HudElement::BottomLeft => {
            set_text(&mut text, &hud.bottom_left_text);
        }
        HudElement::Center => {
            set_text(&mut text, &hud.center_text);
        }
        HudElement::TopRight => {
            set_text(&mut text, &hud.top_right_text);
        }
        HudElement::BottomCenter => {
            set_text(&mut text, &hud.bottom_center_text);
        }
        HudElement::Console => {
            set_text(&mut text, &hud.console_text);
        }
        
        _ => {}
    });
}

fn update_color(
    hud: ResMut<Hud>,
    query: Query<(&mut Handle<ColorMaterial>, &HudElement, &Node)>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
            HudElement::Background => {
                if let Some(material) = materials.get_mut(color_material_handle.clone()) {
                    material.color = hud.background;
                }
            }
            _ => {}
        }
    });
}

fn update_console(
    mut hud: ResMut<Hud>,
    query: Query<(&mut Visible, &HudElement)>,
    console: Res<Console>,
) {
    if console.log != hud.console_text {
        hud.console_text = console.log.clone();
    }

    if hud.is_changed() {
        query.for_each_mut(|(mut visible, element)| {
            if *element == HudElement::Console {
                visible.is_visible = hud.show_console;
            }
        });
    }
}

fn fade_out_in_out(mut hud: ResMut<Hud>, time: Res<Time>) {
    if let Some(mut fade_in_out) = hud.fade_in_out {
        fade_in_out.elapsed_sec += time.delta_seconds();
        match fade_in_out.direction {
            FadeDirection::In => {
                if fade_in_out.elapsed_sec >= fade_in_out.time_in_sec {
                    fade_in_out.elapsed_sec = 0.0;
                    fade_in_out.direction = FadeDirection::InBetween;
                }

                hud.fade_in_out = Some(fade_in_out);
            }
            FadeDirection::Out => {
                if fade_in_out.elapsed_sec >= fade_in_out.time_out_sec {
                    fade_in_out.elapsed_sec = fade_in_out.time_out_sec;
                    hud.fade_in_out = None;
                } else {
                    hud.fade_in_out = Some(fade_in_out);
                }
            }
            FadeDirection::InBetween => {
                if fade_in_out.elapsed_sec >= fade_in_out.time_in_between_sec {
                    fade_in_out.elapsed_sec = 0.0;
                    fade_in_out.direction = FadeDirection::Out;
                }

                hud.fade_in_out = Some(fade_in_out);
            }
        }

        hud.foreground = fade_in_out.base_color;
        hud.foreground.set_a(fade_in_out.alpha());
    }
}


fn egui_hud_system(egui_context: ResMut<EguiContext>, windows: Res<Windows>, hud:Res<Hud>) {
    if let Some(primary) = windows.get_primary() {
        let w = primary.width();
        let h = primary.height();
        let _scale_factor = primary.scale_factor();

        let margin = 10.0;

        egui::Area::new("HudCenter")
        .fixed_pos([margin, margin])
        .show(egui_context.ctx(), |ui| {
            ui.set_width(w - margin * 2.0);
            ui.add_space(h/2.0);
            ui.vertical_centered(|ui| {
                ui.heading(hud.center_text.clone());
            });
        });

        egui::Area::new("HudLeft")
        .fixed_pos([margin, margin])
        .show(egui_context.ctx(), |ui| {
            ui.set_width(w);
            ui.label(hud.top_left_text.clone());
        });
    }
}

pub struct HudPlugin; 

fn update_fps(query: Query<(&mut Text, &FPSText)>, time: Res<Time>) {
    query.for_each_mut(|(mut text, _)| {
        if let Some(section) = text.sections.get_mut(0) {
            section.value = format!("{}", (1.0 / time.delta_seconds()) as u32);
        }
    });
}

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Hud::default());
       // app.add_startup_system(hud_initialization_system.system());
       // app.add_system(update_text.system());
        app.add_system(egui_hud_system.system());
       // app.add_system(update_console.system());
       // app.add_system(fade_out_in_out.system().before("update_foreground"));
       // app.add_system(update_color.system().label("update_foreground"));
        app.add_system(
            update_fps
                .system()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0)),
        );
    }
}
