use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BuildingType {
    #[default]
    House,
    HeadQuarter {
        index: usize,
    },
    Tower,
}

impl BuildingType {
    pub fn grid_size(&self) -> UVec2 {
        match self {
            BuildingType::House => UVec2::splat(3),
            BuildingType::HeadQuarter { index: _ } => UVec2::splat(4),
            BuildingType::Tower => UVec2::splat(2),
        }
    }
}
