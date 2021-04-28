use bevy::prelude::*;
use rand::random;

use crate::{Bot, Tank, Velocity};

pub fn bot_system(mut query:Query<(&mut Tank,&mut Velocity,&mut Bot)>, time:Res<Time>) {
    query.for_each_mut(|(tank, mut thrust, mut bot)| {
        let time = time.seconds_since_startup();
        if bot.next_think <= time {
            bot.next_think = time + 1.0;
            if random::<f32>() > 0.5 {
                thrust.velocity.x = random::<f32>() * 2.0 - 0.5;
                thrust.velocity.y = 0.0;
            } else {
                thrust.velocity.x = 0.0;
                thrust.velocity.y = random::<f32>() * 2.0 - 0.5;
            }
            
        }
    });
}