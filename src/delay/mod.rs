use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::{ecs::component::Component, prelude::*};

pub struct Delay<T> {
    pub remaining:f32,
    pub start:f32,
    pub next_state:Option<T>
}

impl<T : PartialEq> Delay<T> {
    pub fn set_next_state(&mut self, state:T, delay:f32) {
        self.next_state = Some(state);
        self.remaining = delay;
        self.start = delay;
    }

    pub fn some(&self) -> bool {
        self.next_state != None
    }
}

impl<T> Default for Delay<T> {
    fn default() -> Self {
        Self {
            remaining:0.0,
            next_state:None,
            start:1.0
        }
    }
}

fn delay_update<T : Component + Clone + Eq + Debug + Hash>(mut delay:ResMut<Delay<T>>, mut state:ResMut<State<T>>, time:Res<Time>) {
    if let Some(s) = delay.next_state.clone() {
        delay.remaining -= time.delta_seconds();
        if delay.remaining <= 0.0 {
            delay.remaining = 0.0;
            state.overwrite_set(s).unwrap();
        }
    }
}

#[derive(Default)]
pub struct DelayPlugin<T : Component + Clone + Eq + Hash + Debug> {
    phantom:PhantomData<T>
}

impl<T : Component + Clone + Eq + Hash + Debug> Plugin for DelayPlugin<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Delay::<T>::default());
        app.add_system_to_stage(CoreStage::PreUpdate, delay_update::<T>.system());
    }
}