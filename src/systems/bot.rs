use bevy::prelude::*;
use rand::random;

use crate::{Bot, BotState, Tank};

pub fn bot_system(bots:Query<(Entity, &mut Bot, &mut Tank)>, time:Res<Time>) {
    bots.for_each_mut(|(entity, mut bot, mut tank)| {
        let t = time.time_since_startup().as_secs_f64();
        if bot.next_think <= t {
            bot.next_think = t + 0.5;
            
            match bot.state {
                crate::BotState::Idle => {
                    bot.state = BotState::RandomRotate;
                    tank.tracks = [0.0, 0.0];
                }
                crate::BotState::RandomRotate => {
                    let r = random::<f32>() - 0.5;
                    tank.tracks = [r, 0.0];
                }
                crate::BotState::Exploring => {
                    
                }
            }
        }
    });
}