use bevy::prelude::*;

use crate::game::{
    building::{
        self, Building, BuildingProps, building_types::BuildingType, inhabitants::Inhabitants,
    },
    player::PlayerColor,
    projectiles::projectile_launcher::ProjectileLauncher,
};

pub const TOWER_MAX_INHABITANTS: i32 = 30;

pub const TOWER_RANGE: f32 = 200.0;
pub const TOWER_RELOAD_TIME: f32 = 1.0;

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
        let building_type = BuildingType::Tower;
        let image_path = building::asset_paths::get_path(building_type, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, TOWER_MAX_INHABITANTS, None)
            template_value(building_type.base_stats())

            Sprite {
                image: image_path
            }

            ProjectileLauncher::new(TOWER_RANGE, TOWER_RELOAD_TIME)
        }
    }

    pub const fn _max_inhabitants() -> i32 {
        TOWER_MAX_INHABITANTS
    }

    pub const fn inhabitants_percentage(current: i32) -> f32 {
        current as f32 / TOWER_MAX_INHABITANTS as f32
    }
}
