use bevy::prelude::*;
use bevy_rapier2d::{physics::{ColliderHandleComponent, RigidBodyHandleComponent}, rapier::{dynamics::{RigidBodySet}, geometry::{ColliderSet, InteractionGroups, NarrowPhase, Ray}, pipeline::QueryPipeline}};
use rand::random;

use crate::{Bot, BotState, Tank};

pub fn bot_sensor_system(bots:Query<&mut Bot>, tanks:Query<&Tank>, rigid_bodies:Query<&RigidBodyHandleComponent>, colliders:Query<&ColliderHandleComponent>) {
    bots.for_each_mut(|bot| {
        println!("bot sensing");
    });
}


pub fn bot_system(bots:Query<(Entity, &mut Bot, &mut Tank, &RigidBodyHandleComponent)>, time:Res<Time>, bodies:Res<RigidBodySet>, query_pipeline:Res<QueryPipeline>, collider_set:Res<ColliderSet>) {
    bots.for_each_mut(|(entity, mut bot, mut tank, body)| {
        let t = time.time_since_startup().as_secs_f64();
        if let Some(body) = bodies.get(body.handle()) {
                if bot.next_think <= t {
                    bot.next_think = t + 0.1;
                    match bot.state {
                        BotState::Idle => {
                            bot.state = BotState::RandomRotate;
                            tank.tracks = [0.0, 0.0];
                        }
                        BotState::RandomRotate => {
                            if bot.mem[0] == 0.0 {
                                let r = random::<f32>() - 0.5;
                                if r > 0.0 {
                                    tank.tracks = [1.0, -1.0];
    
                                } 
                                else {
                                    tank.tracks = [-1.0, 1.0];
                                }
                            }

                            bot.mem[0] += 1.0;
                            if bot.mem[0] > 10.0 {
                                bot.state = BotState::Exploring;  

                            }
                        }
                        BotState::Exploring => {
                            tank.tracks = [1.0, 1.0];
                            if raycast_front(body, &query_pipeline, &collider_set) != None {
                                // front of tank hit something
                                bot.state = BotState::RandomRotate;
                                bot.mem[0] = 0.0;
                            }
                            else {
                                if raycast_target(Vec2::default(), body, &query_pipeline, &collider_set) != None {
                                    // target is free
                                    println!("free");
                                } else {
                                    println!("not free");
                                }
                            }
                        }
                        BotState::Rotate180 => {
                            if bot.mem[0] > 0.0 {
                                tank.tracks = [1.0, -1.0];
                                bot.mem[0] -= 1.0;
                                println!("rotate180");
                            } else {
                                bot.mem[0];
                                bot.state = BotState::Idle;
                            }
                        }
                    }
                }

        }

    });
}

fn raycast_front(body: &bevy_rapier2d::rapier::dynamics::RigidBody, query_pipeline: &Res<QueryPipeline>, collider_set: &Res<ColliderSet>) -> Option<(bevy_rapier2d::rapier::geometry::ColliderHandle, f32)> {
    let o:Vec2 = [body.position().translation.x, body.position().translation.y].into();
    let dir:Vec2 = [body.position().rotation.re, body.position().rotation.im].into();
    let o = o + dir;
    let ray = Ray { 
        origin:[o.x, o.y].into(),
        dir:[dir.x, dir.y].into()
    };
    let res = query_pipeline.cast_ray(&collider_set, 
        &ray, 1.0, true, InteractionGroups::default(), None);
    res
}


fn raycast_target(target:Vec2, body: &bevy_rapier2d::rapier::dynamics::RigidBody, query_pipeline: &Res<QueryPipeline>, collider_set: &Res<ColliderSet>) -> Option<(bevy_rapier2d::rapier::geometry::ColliderHandle, f32)> {
    let o:Vec2 = [body.position().translation.x, body.position().translation.y].into();
    let dir:Vec2 = target - o;
    if dir.length() == 0.0 {
        return None;
    }

    let dir = dir.normalize();
    let o = o + dir;
    let ray = Ray { 
        origin:[o.x, o.y].into(),
        dir:[dir.x, dir.y].into()
    };
    let res = query_pipeline.cast_ray(&collider_set, 
        &ray, 1.0, true, InteractionGroups::default(), None);
    res
}