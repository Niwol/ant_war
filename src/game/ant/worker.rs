use bevy::prelude::*;

use crate::game::{
    ant::{
        self, Ant, AntProps,
        ant_stats::{AntStats, Health},
        ant_type::AntType,
    },
    player::PlayerColor,
};

const WORKER_SPEED: f32 = 70.0;
const WORKER_ATTACK_POWER: f32 = 0.8;
const WORKER_HEALTH: f32 = 1.0;

#[derive(SceneComponent, Default, Clone)]
#[scene(WorkerProps)]
pub struct Worker;

#[derive(Default)]
pub struct WorkerProps {
    pub ant_props: AntProps,
    pub player_color: PlayerColor,
}

impl Worker {
    fn scene(props: WorkerProps) -> impl Scene {
        let ant_type = AntType::Worker;
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
                speed: WORKER_SPEED,
                attack_power: WORKER_ATTACK_POWER,
                health: Health {
                    max: WORKER_HEALTH,
                    current: WORKER_HEALTH,
                }
            }
        }
    }
}
