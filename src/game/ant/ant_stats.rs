use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct AntStats {
    pub speed: f32,
    pub attack_power: f32,
    pub health: f32,
}
