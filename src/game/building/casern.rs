use bevy::prelude::*;

use crate::game::{
    building::{Building, BuildingProps, building_type::BuildingType, inhabitants::Inhabitants},
    player::PlayerColor,
};

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
        let image_path = super::asset_paths::get_path(BuildingType::Casern, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type: BuildingType::Casern,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, 20, Some(Timer::from_seconds(5.0, TimerMode::Repeating)))

            Sprite {
                image: image_path
            }
        }
    }
}
