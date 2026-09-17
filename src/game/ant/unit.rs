use bevy::prelude::*;

use crate::game::{
    ant::{self, Ant, AntProps, ant_type::AntType},
    player::PlayerColor,
};

pub const UNIT_SPEED: f32 = 50.0;
pub const UNIT_ATTACK_POWER: f32 = 1.0;
pub const UNIT_HEALTH: f32 = 1.0;

#[derive(SceneComponent, Default, Clone)]
#[scene(UnitProps)]
pub struct Unit;

#[derive(Default)]
pub struct UnitProps {
    pub ant_props: AntProps,
    pub player_color: PlayerColor,
}

impl Unit {
    fn scene(props: UnitProps) -> impl Scene {
        let ant_type = AntType::Unit;
        let ant_props = props.ant_props;
        let image_path = ant::asset_paths::get_path(ant_type, props.player_color);

        bsn! {
            @Ant {
                _ant_type: ant_type,
                @position: {ant_props.position},
            }

            Sprite {
                image: image_path
            }

            template_value(AntType::Unit.base_stats())
        }
    }
}
