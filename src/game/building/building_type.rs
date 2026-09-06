use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::ant::ant_type::AntType;

#[derive(Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum BuildingType {
    #[default]
    House,
    HeadQuarter {
        index: usize,
    },
    Tower,
    Casern,
    Walls,
}

impl BuildingType {
    pub fn grid_size(&self) -> UVec2 {
        match self {
            BuildingType::House => UVec2::splat(3),
            BuildingType::HeadQuarter { index: _ } => UVec2::splat(4),
            BuildingType::Tower => UVec2::splat(2),
            BuildingType::Casern => UVec2::splat(3),
            BuildingType::Walls => UVec2::splat(5),
        }
    }

    pub fn name(&self) -> String {
        match self {
            BuildingType::House => "House",
            BuildingType::HeadQuarter { index: _ } => "Head Quarter",
            BuildingType::Tower => "Tower",
            BuildingType::Casern => "Casern",
            BuildingType::Walls => "Walls",
        }
        .to_string()
    }

    pub fn ants_produced(&self) -> AntType {
        match self {
            BuildingType::House => AntType::Unit,
            BuildingType::HeadQuarter { index: _ } => AntType::Worker,
            BuildingType::Tower => AntType::Unit,
            BuildingType::Casern => AntType::Soldier,
            BuildingType::Walls => AntType::Unit,
        }
    }
}
