use bevy::prelude::*;

use crate::game::{
    building::{
        self, Building, BuildingProps, building_types::BuildingType, inhabitants::Inhabitants,
    },
    player::PlayerColor,
};

pub const HQ_MAX_INHABITANTS: i32 = 40;

pub fn plugin(_app: &mut App) {}

#[derive(SceneComponent, Default, Clone)]
#[scene(HeadQuarterProps)]
pub struct HeadQuarter;

#[derive(Default)]
pub struct HeadQuarterProps {
    pub building_props: BuildingProps,
    pub main_building_index: usize,
}

impl HeadQuarter {
    fn scene(props: HeadQuarterProps) -> impl Scene {
        let building_props = props.building_props;
        let building_type = BuildingType::HeadQuarter {
            index: props.main_building_index,
        };
        let image_path = building::asset_paths::get_path(building_type, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, HQ_MAX_INHABITANTS, Some(Timer::from_seconds(1.0, TimerMode::Repeating)))
            template_value(building_type.base_stats())

            Sprite {
                image: image_path
            }
        }
    }

    pub const fn _max_inhabitants() -> i32 {
        HQ_MAX_INHABITANTS
    }

    pub const fn inhabitants_percentage(current: i32) -> f32 {
        current as f32 / HQ_MAX_INHABITANTS as f32
    }
}
