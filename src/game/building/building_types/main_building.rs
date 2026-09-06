use bevy::prelude::*;

use crate::game::{
    building::{
        self, Building, BuildingProps, building_types::BuildingType, inhabitants::Inhabitants,
    },
    player::PlayerColor,
};

pub fn plugin(_app: &mut App) {}

#[derive(SceneComponent, Default, Clone)]
#[scene(MainBuildingProps)]
pub struct MainBuilding;

#[derive(Default)]
pub struct MainBuildingProps {
    pub building_props: BuildingProps,
    pub main_building_index: usize,
}

impl MainBuilding {
    fn scene(props: MainBuildingProps) -> impl Scene {
        let building_props = props.building_props;
        let image_path = building::asset_paths::get_path(
            BuildingType::HeadQuarter { index: 0 },
            PlayerColor::Neutral,
        );

        bsn! {
            @Building {
                building_type: BuildingType::HeadQuarter  {
                    index: {props.main_building_index}
                }
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, 40, Some(Timer::from_seconds(1.0, TimerMode::Repeating)))

            Sprite {
                image: image_path
            }
        }
    }
}
