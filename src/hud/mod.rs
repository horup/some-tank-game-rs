use bevy::{core::FixedTimestep, prelude::*};
use bevy_egui::{EguiContext, egui::{self, Align, Color32, FontDefinitions, FontFamily, Label, Layout, Order, Pos2, Rgba, TextStyle}};
use bevy_egui::egui::Rect;

use crate::{Config};

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
    pub stack:Vec<Hud>,
    pub fps_text:String,
    pub font_size:f32
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
            stack:Vec::new(),
            fps_text:"".into(),
            font_size:0.0
        }
    }
}

fn egui_hud_system(config:Res<Config>, egui_context: ResMut<EguiContext>, windows: Res<Windows>, mut hud:ResMut<Hud>, time: Res<Time>) {
    if let Some(primary) = windows.get_primary() {
        let scale_factor = primary.scale_factor();
        let margin = 10.0;
        let w = primary.width() - margin * 2.0;
        let h = primary.height() - margin * 2.0;

        update_fonts(scale_factor, &egui_context, &mut hud);
        let s = hud.font_size;
        let color = Color32::WHITE;

        egui::Area::new("HudCenter")
        .fixed_pos([margin, margin])
        .show(egui_context.ctx(), |ui| {
            ui.set_width(w);
            ui.add_space(h/2.0 - s);
            ui.vertical_centered(|ui| {
                ui.add(Label::new(hud.center_text.clone()).text_color(color).heading());
                ui.add(Label::new(hud.bottom_center_text.clone()).text_color(color).text_style(TextStyle::Button));
            });
        });

        egui::Area::new("HudLeft")
        .fixed_pos([margin, margin])
        .show(egui_context.ctx(), |ui| {
            ui.set_width(w);
            ui.colored_label(color, hud.top_left_text.clone());
        });

        egui::Area::new("HudBottomLeft")
        .fixed_pos([margin, margin])
        .show(egui_context.ctx(), |ui| {
            ui.set_width(w);
            ui.set_height(h);
            ui.with_layout(Layout::bottom_up(Align::Min), |ui| {
                ui.colored_label(color, hud.bottom_left_text.clone());
            });
        });

        egui::Area::new("HudRight")
        .fixed_pos([margin, margin])
        .show(egui_context.ctx(), |ui| {
            ui.set_width(w);
            ui.with_layout(Layout::with_cross_align(*ui.layout(), Align::Max), |ui| {
                ui.colored_label(color, hud.top_right_text.clone());
            });
        });

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
            
            egui::Area::new("Foreground")
            .fixed_pos([0.0, 0.0])
            .order(Order::Foreground)
            .show(egui_context.ctx(), |ui| {
                ui.set_width(primary.width());
                ui.set_height(primary.height());
                let c = hud.foreground.as_rgba_f32();
                ui.painter().rect_filled(Rect {
                    min:Pos2::new(0.0, 0.0),
                    max:Pos2::new(primary.width(), primary.height())
                }, 0.0, Rgba::from_rgba_premultiplied(c[0], c[1], c[2], c[3]));
            });
        }
        
        if config.show_fps() {
            egui::Area::new("FPS")
            .fixed_pos([margin, margin])
            .order(Order::Tooltip)
            .show(egui_context.ctx(), |ui| {
                ui.set_width(w);
                ui.with_layout(Layout::with_cross_align(*ui.layout(), Align::Max), |ui| {
                    ui.add(Label::new(hud.fps_text.clone()).text_color(Color32::RED).monospace());
                });
            });
        }
    }
}

fn update_fonts(scale_factor: f64, egui_context: &ResMut<EguiContext>, hud:&mut Hud) {
    let font_size = 16.0 / scale_factor as f32;
    if hud.font_size != font_size {
        let mut fonts = FontDefinitions::default();
        fonts.family_and_size.insert(
            TextStyle::Body,
            (FontFamily::Proportional, font_size)
        );
        fonts.family_and_size.insert(
            TextStyle::Heading,
            (FontFamily::Proportional, font_size * 2.0)
        );
        fonts.family_and_size.insert(
            TextStyle::Button,
            (FontFamily::Proportional, font_size * 1.5)
        );
        egui_context.ctx().set_fonts(fonts);
        hud.font_size = font_size;
    }
}

pub struct HudPlugin; 

fn update_fps(mut hud:ResMut<Hud>, time: Res<Time>) {
    hud.fps_text = format!("{}", (1.0 / time.delta_seconds()) as u32);
}

impl Plugin for HudPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Hud::default());
        app.add_system(egui_hud_system.system());
        app.add_system(
            update_fps
                .system()
                .with_run_criteria(FixedTimestep::steps_per_second(1.0)),
        );
    }
}
