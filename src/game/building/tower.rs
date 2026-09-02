use bevy::prelude::*;

use crate::game::{
    building::{Building, BuildingProps, BuildingType, inhabitants::Inhabitants},
    player::PlayerColor,
};

pub fn plugin(_app: &mut App) {}

#[derive(SceneComponent, Default, Clone)]
#[scene(TowerProps)]
pub struct Tower;

#[derive(Default)]
pub struct TowerProps {
    pub building_props: BuildingProps,
}

impl Tower {
    fn scene(props: TowerProps) -> impl Scene {
        let building_props = props.building_props;
        let image_path = super::asset_paths::get_path(BuildingType::Tower, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type: BuildingType::Tower,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, 30, None)

            Sprite {
                image: image_path
            }
        }
    }
}
