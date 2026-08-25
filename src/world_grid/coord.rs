use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world_grid::CELL_SIZE;

#[derive(Serialize, Deserialize, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord(pub IVec2);

impl Coord {
    pub fn from_world(world: Vec2) -> Self {
        Self(IVec2 {
            x: (world.x / CELL_SIZE).floor() as i32,
            y: (world.y / CELL_SIZE).floor() as i32,
        })
    }

    pub fn to_world(&self) -> Vec2 {
        Vec2 {
            x: self.0.x as f32 * CELL_SIZE,
            y: self.0.y as f32 * CELL_SIZE,
        }
    }
}
