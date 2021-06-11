use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::{ecs::component::Component, prelude::*};

pub struct DelayState<T> {
    pub remaining:f32,
    pub start:f32,
    pub next_state:Option<T>
}

impl<T : Component + Clone + Eq + Hash + Debug> DelayState<T> {
    
    // set next state and delay.
    // if existing state and delay has been set, simply ignore
    pub fn set(&mut self, state:T, delay:f32) -> bool {
        if self.has_state() {
            return false;
        }

        self.next_state = Some(state);
        self.remaining = delay;
        self.start = delay;
        true
    }

    pub fn has_state(&self) -> bool {
        self.next_state != None
    }

    pub fn elapsed_normalized(&self) -> f32 {
        if self.start <= 0.0 {
            return 0.0;
        }

        let elapsed =  1.0 - self.remaining / self.start;
        return elapsed;
    }
}

impl<T> Default for DelayState<T> {
    fn default() -> Self {
        Self {
            remaining:0.0,
            next_state:None,
            start:1.0
        }
    }
}

fn delay_update<T : Component + Clone + Eq + Debug + Hash>(mut delay:ResMut<DelayState<T>>, mut state:ResMut<State<T>>, time:Res<Time>) {
    if let Some(s) = delay.next_state.clone() {
        delay.remaining -= time.delta_seconds();
        if delay.remaining <= 0.0 {
            delay.next_state = None;
            delay.remaining = 0.0;
            let _ = state.overwrite_set(s);
        }
    }
}

#[derive(Default)]
pub struct DelayPlugin<T : Component + Clone + Eq + Hash + Debug> {
    phantom:PhantomData<T>
}

impl<T : Component + Clone + Eq + Hash + Debug> Plugin for DelayPlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(DelayState::<T>::default());
        app.add_system_to_stage(CoreStage::PreUpdate, delay_update::<T>.system());
    }
}