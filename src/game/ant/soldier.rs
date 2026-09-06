use bevy::prelude::*;

use crate::game::{
    ant::{self, Ant, AntProps, ant_stats::AntStats, ant_type::AntType},
    player::PlayerColor,
};

const SOLDIER_SPEED: f32 = 40.0;
const SOLDIER_ATTACK_POWER: f32 = 1.5;
const SOLDIER_HEALTH: f32 = 1.5;

#[derive(SceneComponent, Default, Clone)]
#[scene(SoldierProps)]
pub struct Soldier;

#[derive(Default)]
pub struct SoldierProps {
    pub ant_props: AntProps,
    pub player_color: PlayerColor,
}

impl Soldier {
    fn scene(props: SoldierProps) -> impl Scene {
        let ant_type = AntType::Soldier;
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

            AntStats {
                speed: SOLDIER_SPEED,
                attack_power: SOLDIER_ATTACK_POWER,
                health: SOLDIER_HEALTH,
            }
        }
    }
}
