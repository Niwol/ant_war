use bevy::prelude::*;

#[derive(Component, Default, Clone)]
pub struct AntStats {
    pub speed: f32,
    pub attack_power: f32,
    pub health: Health,
}

#[derive(Default, Clone, Copy)]
pub struct Health {
    pub max: f32,
    pub current: f32,
}

impl Health {
    pub fn is_dead(&self) -> bool {
        self.current <= 0.0
    }
}
