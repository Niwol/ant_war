use bevy::prelude::*;

use crate::game::{
    building::{
        self, Building, BuildingProps, building_types::BuildingType, inhabitants::Inhabitants,
    },
    player::PlayerColor,
};

pub const CASERN_MAX_INHABITANTS: i32 = 20;

pub fn plugin(_app: &mut App) {}

#[derive(SceneComponent, Default, Clone)]
#[scene(CasernProps)]
pub struct Casern;

#[derive(Default)]
pub struct CasernProps {
    pub building_props: BuildingProps,
}

impl Casern {
    fn scene(props: CasernProps) -> impl Scene {
        let building_props = props.building_props;
        let building_type = BuildingType::Casern;

        let image_path = building::asset_paths::get_path(building_type, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, CASERN_MAX_INHABITANTS, Some(Timer::from_seconds(5.0, TimerMode::Repeating)))
            template_value(building_type.base_stats())

            Sprite {
                image: image_path
            }
        }
    }

    pub const fn _max_inhabitants() -> i32 {
        CASERN_MAX_INHABITANTS
    }

    pub const fn inhabitants_percentage(current: i32) -> f32 {
        current as f32 / CASERN_MAX_INHABITANTS as f32
    }
}
