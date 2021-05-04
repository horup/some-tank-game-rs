use bevy::prelude::*;
use rand::random;

use crate::{Bot, Tank, Thrust};

pub fn bot_system(mut query:Query<(&mut Tank,&mut Thrust,&mut Bot)>, time:Res<Time>) {
    query.for_each_mut(|(tank, mut thrust, mut bot)| {
        let time = time.seconds_since_startup();
        if bot.next_think <= time {
            bot.next_think = time + 1.0;
            if random::<f32>() > 0.5 {
                thrust.impulse.x = random::<f32>() * 2.0 - 0.5;
                thrust.impulse.y = 0.0;
            } else {
                thrust.impulse.x = 0.0;
                thrust.impulse.y = random::<f32>() * 2.0 - 0.5;
            }
            
        }
    });
}