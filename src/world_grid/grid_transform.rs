use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::world_grid::{CELL_SIZE, coord::Coord};

#[derive(Component, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct GridTransform {
    size: UVec2,
    bottom_left_coord: Coord,
}

impl Default for GridTransform {
    fn default() -> Self {
        Self {
            size: UVec2::ONE,
            bottom_left_coord: Coord::default(),
        }
    }
}

impl GridTransform {
    pub fn new(size: UVec2) -> Self {
        Self {
            size,
            bottom_left_coord: Coord::default(),
        }
    }

    pub fn center_in_world(&self) -> Vec2 {
        let local_center = self.local_center();

        Vec2 {
            x: self.bottom_left_coord.to_world().x + local_center.x,
            y: self.bottom_left_coord.to_world().y + local_center.y,
        }
    }

    pub fn local_center(&self) -> Vec2 {
        Vec2 {
            x: (CELL_SIZE * self.size.x as f32) / 2.0,
            y: (CELL_SIZE * self.size.y as f32) / 2.0,
        }
    }

    pub fn update_from_world(&mut self, world: Vec2) {
        let center = self.local_center();
        let bottom_left = (world - center) + Vec2::splat(CELL_SIZE / 2.0);

        self.bottom_left_coord = Coord::from_world(bottom_left);
    }

    pub fn get_coords(&self) -> Vec<Coord> {
        let mut coords = Vec::new();

        let bottom_left = self.bottom_left_coord;
        for i in 0..self.size.x {
            for j in 0..self.size.y {
                let coord = Coord(IVec2 {
                    x: bottom_left.0.x + i as i32,
                    y: bottom_left.0.y + j as i32,
                });

                coords.push(coord);
            }
        }

        coords
    }
}
