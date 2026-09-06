use bevy::prelude::*;

use crate::{
    game::{
        building::{
            Building, BuildingProps, building_type::BuildingType, inhabitants::Inhabitants,
        },
        player::PlayerColor,
        projectiles::projectile_launcher::ProjectileLauncher,
    },
    world_grid::CELL_SIZE,
};

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
        let image_path = super::asset_paths::get_path(BuildingType::Walls, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type: BuildingType::Walls,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, 50, None)

            ProjectileLauncher::new(WALLS_RANGE, WALLS_RELOAD_TIME)

            Sprite {
                image: image_path
            }
        }
    }
}
