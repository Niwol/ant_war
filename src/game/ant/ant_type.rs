use crate::game::ant::{
    ant_stats::{AntStats, Health},
    soldier::{SOLDIER_ATTACK_POWER, SOLDIER_HEALTH, SOLDIER_SPEED},
    unit::{UNIT_ATTACK_POWER, UNIT_HEALTH, UNIT_SPEED},
    worker::{WORKER_ATTACK_POWER, WORKER_HEALTH, WORKER_SPEED},
};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AntType {
    #[default]
    Unit,
    Worker,
    Soldier,
}

impl AntType {
    pub fn base_stats(&self) -> AntStats {
        match self {
            AntType::Unit => AntStats {
                speed: UNIT_SPEED,
                attack_power: UNIT_ATTACK_POWER,
                health: Health {
                    max: UNIT_HEALTH,
                    current: UNIT_HEALTH,
                },
            },
            AntType::Worker => AntStats {
                speed: WORKER_SPEED,
                attack_power: WORKER_ATTACK_POWER,
                health: Health {
                    max: WORKER_HEALTH,
                    current: WORKER_HEALTH,
                },
            },
            AntType::Soldier => AntStats {
                speed: SOLDIER_SPEED,
                attack_power: SOLDIER_ATTACK_POWER,
                health: Health {
                    max: SOLDIER_HEALTH,
                    current: SOLDIER_HEALTH,
                },
            },
        }
    }
}
