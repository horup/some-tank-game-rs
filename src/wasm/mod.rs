use bevy::{input::InputSystem, prelude::*};
use bevy::window::WindowResized;
use wasm_bindgen::prelude::*;
use web_sys::{TouchEvent, TouchList};

use crate::mouse::{Mouse, MouseSystem};


#[wasm_bindgen]
extern "C" {
    fn resize_canvas(width:f32, height:f32);
}


#[wasm_bindgen]
extern "C" {
    fn has_touch() -> bool;
}

#[wasm_bindgen]
extern "C" {
    fn pop_touch_event() -> Option<TouchEvent>;
}

struct LastSize {
    pub width:f32,
    pub height:f32
}

fn resizer(mut windows:ResMut<Windows>, mut window_resized_events: EventWriter<WindowResized>, wd:Res<WindowDescriptor>, mut last_size:ResMut<LastSize>) {
    let window = web_sys::window().expect("no global `window` exists");
    let mut width:f32 = window.inner_width().unwrap().as_f64().unwrap() as f32;
    let mut height:f32 = window.inner_height().unwrap().as_f64().unwrap() as f32;

    if let Some(window) = windows.get_primary_mut() {
            if width != last_size.width || height != last_size.height {
                *last_size = LastSize {
                    width, height
                };

                width = if width < wd.resize_constraints.min_width { wd.resize_constraints.min_width } else { width };
                width = if width > wd.resize_constraints.max_width { wd.resize_constraints.max_width } else { width };
                height = if height < wd.resize_constraints.min_height { wd.resize_constraints.min_height } else { height };
                height = if height > wd.resize_constraints.max_height { wd.resize_constraints.max_height } else { height };

                let p_width = width * window.scale_factor() as f32;
                let p_height = height * window.scale_factor() as f32;
                window.update_actual_size_from_backend(p_width as u32, p_height as u32);
                window_resized_events.send(WindowResized {
                    id:window.id(),
                    height:height,
                    width:width
                });

                resize_canvas(width, height);
                info!("Resizing to {:?},{:?} with scale factor of {}", width, height, window.scale_factor());
            }

    }
}

fn pool_touch_system(mut mouse:ResMut<Mouse>, windows:Res<Windows>, mut mouse_button_input: ResMut<Input<MouseButton>>,) {
    if let Some(window) = windows.get_primary() {
        while let Some(touch_event) = pop_touch_event() {
            let t = touch_event.type_();
            let touches:TouchList = touch_event.touches();

            for i in 0..touches.length() {
                if let Some(touch) = touches.get(i) {
                    let _id = touch.identifier();
                    let x = touch.client_x() as f32;
                    let y = window.height() as f32 - touch.client_y() as f32;
                    mouse.pos_screen = Vec2::new(x, y);
                }
            }

            if t == "touchstart" {
                mouse_button_input.press(MouseButton::Left);
                info!("start");
            } else if t ==  "touchend" {
                mouse_button_input.release(MouseButton::Left);
                info!("end");
            } else if t ==  "touchmove" {
               
            }
        }
    }
}


pub struct WASMPlugin;

impl Plugin for WASMPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(bevy_webgl2::WebGL2Plugin);
        app.insert_resource(LastSize { width:0.0, height:0.0});
        app.add_system(resizer.system());

        if has_touch() {
            app.add_system_to_stage(CoreStage::PreUpdate, pool_touch_system.system().after(InputSystem).before(MouseSystem));
        }
    }
}