use std::collections::VecDeque;

use bevy::prelude::*;

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub struct Waypoint {
    pub location:Vec3
}

impl From<Vec3> for Waypoint {
    fn from(v: Vec3) -> Self {
        Self {
            location:v
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Autopilot {
    pub waypoints:VecDeque<Waypoint>,
    pub planning:bool
}

impl Autopilot {
    pub fn clear(&mut self) {
        self.waypoints.clear();
        self.planning = false;
    }

    pub fn any_within_radius(&self, radius:f32, p:Vec3) -> bool {
        for w in &self.waypoints {
            if w.location.distance(p) <= radius {
                return true;
            }
        }
        false
    }
}