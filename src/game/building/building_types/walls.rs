use bevy::prelude::*;

use crate::{
    game::{
        building::{
            self, Building, BuildingProps, building_types::BuildingType, inhabitants::Inhabitants,
        },
        player::PlayerColor,
        projectiles::projectile_launcher::ProjectileLauncher,
    },
    world_grid::CELL_SIZE,
};

pub const WALLS_MAX_INHABITANTS: i32 = 50;

const WALLS_RANGE: f32 = CELL_SIZE * 6.0;
const WALLS_RELOAD_TIME: f32 = 2.0;

pub fn plugin(_app: &mut App) {}

#[derive(SceneComponent, Default, Clone)]
#[scene(WallsProps)]
pub struct Walls;

#[derive(Default)]
pub struct WallsProps {
    pub building_props: BuildingProps,
}

impl Walls {
    fn scene(props: WallsProps) -> impl Scene {
        let building_props = props.building_props;
        let building_type = BuildingType::Walls;
        let image_path = building::asset_paths::get_path(building_type, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type: BuildingType::Walls,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, WALLS_MAX_INHABITANTS, None)
            template_value(building_type.base_stats())
            ProjectileLauncher::new(WALLS_RANGE, WALLS_RELOAD_TIME)

            Sprite {
                image: image_path
            }
        }
    }

    pub const fn _max_inhabitants() -> i32 {
        WALLS_MAX_INHABITANTS
    }

    pub const fn inhabitants_percentage(current: i32) -> f32 {
        current as f32 / WALLS_MAX_INHABITANTS as f32
    }
}
