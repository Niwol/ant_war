use bevy::prelude::*;

use crate::game::{
    building::{
        self, Building, BuildingProps, building_types::BuildingType, inhabitants::Inhabitants,
    },
    player::PlayerColor,
};

pub const HOUSE_MAX_INHABITANTS: i32 = 20;
pub const HOUSE_PRODUCTION_TIME: f32 = 2.0;

pub fn plugin(_app: &mut App) {}

#[derive(SceneComponent, Default, Clone)]
#[scene(HouseProps)]
pub struct House;

#[derive(Default)]
pub struct HouseProps {
    pub building_props: BuildingProps,
}

impl House {
    fn scene(props: HouseProps) -> impl Scene {
        let building_props = props.building_props;
        let building_type = BuildingType::House;
        let image_path = building::asset_paths::get_path(building_type, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, HOUSE_MAX_INHABITANTS, Some(Timer::from_seconds(HOUSE_PRODUCTION_TIME, TimerMode::Repeating)))
            template_value(building_type.base_stats())

            Sprite {
                image: image_path
            }
        }
    }

    pub const fn _max_inhabitants() -> i32 {
        HOUSE_MAX_INHABITANTS
    }

    pub const fn inhabitants_percentage(current: i32) -> f32 {
        current as f32 / HOUSE_MAX_INHABITANTS as f32
    }
}
