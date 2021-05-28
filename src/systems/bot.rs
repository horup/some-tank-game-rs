use bevy::prelude::*;
use bevy_rapier2d::{physics::{RigidBodyHandleComponent}, rapier::{dynamics::{RigidBodySet}, geometry::{ColliderSet, InteractionGroups, Ray}, math::Real, pipeline::QueryPipeline}};
use rand::random;

use crate::{Bot, BotState, Enemy, Tank, Turret};

pub fn bot_sensor_system(tanks:Query<(Entity, &Tank)>, bots:Query<(Entity, &mut Bot)>, rigid_bodies:Query<&RigidBodyHandleComponent>, rigid_body_set:Res<RigidBodySet>, collider_set:Res<ColliderSet>, query_pipeline: Res<QueryPipeline>) {
    bots.for_each_mut(|(bot_entity, mut bot)| {
        if let Ok(bot_body) = rigid_bodies.get_component::<RigidBodyHandleComponent>(bot_entity) {
            if let Some(bot_body) = rigid_body_set.get(bot_body.handle()) {
                // measure distance to front
                bot.sensors.obstacle_distance_front = raycast_front_distance(bot_body, &query_pipeline, &collider_set);

                // collect know enemies
                tanks.for_each(|(tank_entity, tank)| {
                    if tank_entity != bot_entity {
                        if let Ok(enemy_body) = rigid_bodies.get_component::<RigidBodyHandleComponent>(tank_entity) {
                            if let Some(enemy_body) = rigid_body_set.get(enemy_body.handle()) {
                                let pos:Vec3 = [enemy_body.position().translation.x, enemy_body.position().translation.y, 0.0].into();
                                bot.sensors.known_enemies.push(Enemy {
                                    entity:tank_entity,
                                    position:pos
                                });
                            }
                        }
                    }
                });

                // find visibile enemies
                bot.sensors.visible_enemies.clear();
                let sensors = &mut bot.sensors;
                let known_enemies = &sensors.known_enemies;
                let visible_enemies = &mut sensors.visible_enemies;
                for enemy in known_enemies {
                    let test = raycast_target(enemy.position.truncate(), bot_body, &query_pipeline, &collider_set);
                    if let Some((handle, _)) = test {
                        if let Some(collider) = collider_set.get(handle) {
                            let e = Entity::from_bits(collider.user_data as u64);
                            if enemy.entity == e {
                                visible_enemies.push(*enemy);
                            }
                        }
                    }
                }
            }
        }
    });
}


pub fn bot_system(turrets:Query<(Entity, &mut Turret)>, bots:Query<(Entity, &mut Bot, &mut Tank, &RigidBodyHandleComponent, &Children)>, time:Res<Time>, bodies:Res<RigidBodySet>, query_pipeline:Res<QueryPipeline>, collider_set:Res<ColliderSet>) {
    bots.for_each_mut(|(bot_entity, mut bot, mut tank, body, children)| {
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
                            if bot.sensors.obstacle_distance_front < 2.0 {
                                // front of tank hit something
                                bot.state = BotState::RandomRotate;
                                bot.mem[0] = 0.0;
                            }
                            else {
                               // track and shoot enemies
                               for enemy in &bot.sensors.visible_enemies {
                                  // if let Ok(turret) = 
                               }
                            }
                        }
                        BotState::Rotate180 => {
                            if bot.mem[0] > 0.0 {
                                tank.tracks = [1.0, -1.0];
                                bot.mem[0] -= 1.0;
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

fn raycast_front_distance(body: &bevy_rapier2d::rapier::dynamics::RigidBody, query_pipeline: &Res<QueryPipeline>, collider_set: &Res<ColliderSet>) -> f32 {
    let o:Vec2 = [body.position().translation.x, body.position().translation.y].into();
    let dir:Vec2 = [body.position().rotation.re, body.position().rotation.im].into();
    let o = o + dir;
    let ray = Ray { 
        origin:[o.x, o.y].into(),
        dir:[dir.x, dir.y].into()
    };
    let res = query_pipeline.cast_ray(&collider_set, 
        &ray, Real::MAX, true, InteractionGroups::default(), None);
    
    if let Some((_handle, r)) = res {
        return r;
    }

    return f32::MAX;
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
        &ray, f32::MAX, true, InteractionGroups::default(), None);
    res
}