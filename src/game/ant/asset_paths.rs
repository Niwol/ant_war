use crate::game::{ant::ant_type::AntType, player::PlayerColor};

pub const PATH_UNIT_BLUE: &str = "sprites/ants/unit/unit_blue.png";
pub const PATH_UNIT_RED: &str = "sprites/ants/unit/unit_red.png";
pub const PATH_UNIT_GREEN: &str = "sprites/ants/unit/unit_green.png";
pub const PATH_UNIT_ORANGE: &str = "sprites/ants/unit/unit_orange.png";
pub const PATH_UNIT_PURPLE: &str = "sprites/ants/unit/unit_purple.png";
pub const PATH_UNIT_YELLOW: &str = "sprites/ants/unit/unit_yellow.png";
pub const PATH_UNIT_AQUA: &str = "sprites/ants/unit/unit_aqua.png";
pub const PATH_UNIT_PINK: &str = "sprites/ants/unit/unit_pink.png";

pub const PATH_WORKER_BLUE: &str = "sprites/ants/worker/worker_blue.png";
pub const PATH_WORKER_RED: &str = "sprites/ants/worker/worker_red.png";
pub const PATH_WORKER_GREEN: &str = "sprites/ants/worker/worker_green.png";
pub const PATH_WORKER_ORANGE: &str = "sprites/ants/worker/worker_orange.png";
pub const PATH_WORKER_PURPLE: &str = "sprites/ants/worker/worker_purple.png";
pub const PATH_WORKER_YELLOW: &str = "sprites/ants/worker/worker_yellow.png";
pub const PATH_WORKER_AQUA: &str = "sprites/ants/worker/worker_aqua.png";
pub const PATH_WORKER_PINK: &str = "sprites/ants/worker/worker_pink.png";

pub const PATH_SOLDIER_BLUE: &str = "sprites/ants/soldier/soldier_blue.png";
pub const PATH_SOLDIER_RED: &str = "sprites/ants/soldier/soldier_red.png";
pub const PATH_SOLDIER_GREEN: &str = "sprites/ants/soldier/soldier_green.png";
pub const PATH_SOLDIER_ORANGE: &str = "sprites/ants/soldier/soldier_orange.png";
pub const PATH_SOLDIER_PURPLE: &str = "sprites/ants/soldier/soldier_purple.png";
pub const PATH_SOLDIER_YELLOW: &str = "sprites/ants/soldier/soldier_yellow.png";
pub const PATH_SOLDIER_AQUA: &str = "sprites/ants/soldier/soldier_aqua.png";
pub const PATH_SOLDIER_PINK: &str = "sprites/ants/soldier/soldier_pink.png";

pub fn get_path(ant_type: AntType, player_color: PlayerColor) -> &'static str {
    match ant_type {
        AntType::Unit => match player_color {
            PlayerColor::Blue => PATH_UNIT_BLUE,
            PlayerColor::Red => PATH_UNIT_RED,
            PlayerColor::Green => PATH_UNIT_GREEN,
            PlayerColor::Orange => PATH_UNIT_ORANGE,
            PlayerColor::Purple => PATH_UNIT_PURPLE,
            PlayerColor::Yellow => PATH_UNIT_YELLOW,
            PlayerColor::Aqua => PATH_UNIT_AQUA,
            PlayerColor::Pink => PATH_UNIT_PINK,
            PlayerColor::Neutral => unreachable!("No neutral ant color"),
        },

        AntType::Worker => match player_color {
            PlayerColor::Blue => PATH_WORKER_BLUE,
            PlayerColor::Red => PATH_WORKER_RED,
            PlayerColor::Green => PATH_WORKER_GREEN,
            PlayerColor::Orange => PATH_WORKER_ORANGE,
            PlayerColor::Purple => PATH_WORKER_PURPLE,
            PlayerColor::Yellow => PATH_WORKER_YELLOW,
            PlayerColor::Aqua => PATH_WORKER_AQUA,
            PlayerColor::Pink => PATH_WORKER_PINK,
            PlayerColor::Neutral => unreachable!("No neutral ant color"),
        },

        AntType::Soldier => match player_color {
            PlayerColor::Blue => PATH_SOLDIER_BLUE,
            PlayerColor::Red => PATH_SOLDIER_RED,
            PlayerColor::Green => PATH_SOLDIER_GREEN,
            PlayerColor::Orange => PATH_SOLDIER_ORANGE,
            PlayerColor::Purple => PATH_SOLDIER_PURPLE,
            PlayerColor::Yellow => PATH_SOLDIER_YELLOW,
            PlayerColor::Aqua => PATH_SOLDIER_AQUA,
            PlayerColor::Pink => PATH_SOLDIER_PINK,
            PlayerColor::Neutral => unreachable!("No neutral ant color"),
        },
    }
}
