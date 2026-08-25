use bevy::prelude::*;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Cell {
    #[default]
    Empty,
    Occupied {
        entity: Entity,
    },
}
