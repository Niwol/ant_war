use bevy::prelude::*;

use crate::game::{
    building::{
        self, Building, BuildingProps, building_stats::BuildingStats, building_types::BuildingType,
        inhabitants::Inhabitants,
    },
    player::PlayerColor,
};

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
        let image_path = building::asset_paths::get_path(BuildingType::House, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type: BuildingType::House,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, 20, Some(Timer::from_seconds(2.0, TimerMode::Repeating)))

            BuildingStats {
                defense: 1.0,
            }

            Sprite {
                image: image_path
            }
        }
    }
}
