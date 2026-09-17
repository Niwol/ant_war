use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::{
    ant::ant_type::AntType,
    building::{
        building_stats::BuildingStats,
        building_types::{
            casern::Casern, head_quarter::HeadQuarter, house::House, tower::Tower, walls::Walls,
        },
    },
};

pub mod casern;
pub mod head_quarter;
pub mod house;
pub mod tower;
pub mod walls;

pub struct BuildingTypesPlugin;
impl Plugin for BuildingTypesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            head_quarter::plugin,
            house::plugin,
            tower::plugin,
            casern::plugin,
            walls::plugin,
        ));
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize, Default)]
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

    pub const fn _max_inhabitants(&self) -> i32 {
        match self {
            BuildingType::House => House::_max_inhabitants(),
            BuildingType::HeadQuarter { index: _ } => HeadQuarter::_max_inhabitants(),
            BuildingType::Tower => Tower::_max_inhabitants(),
            BuildingType::Casern => Casern::_max_inhabitants(),
            BuildingType::Walls => Walls::_max_inhabitants(),
        }
    }

    pub const fn inhabitants_percentage(&self, current: i32) -> f32 {
        match self {
            BuildingType::House => House::inhabitants_percentage(current),
            BuildingType::HeadQuarter { index: _ } => HeadQuarter::inhabitants_percentage(current),
            BuildingType::Tower => Tower::inhabitants_percentage(current),
            BuildingType::Casern => Casern::inhabitants_percentage(current),
            BuildingType::Walls => Walls::inhabitants_percentage(current),
        }
    }

    pub const fn base_stats(&self) -> BuildingStats {
        match self {
            BuildingType::House => BuildingStats { defense: 1.0 },
            BuildingType::HeadQuarter { .. } => BuildingStats { defense: 1.0 },
            BuildingType::Tower => BuildingStats { defense: 1.0 },
            BuildingType::Casern => BuildingStats { defense: 1.0 },
            BuildingType::Walls => BuildingStats { defense: 1.5 },
        }
    }
}
