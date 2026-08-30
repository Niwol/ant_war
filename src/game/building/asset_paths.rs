use crate::game::{building::BuildingType, player::PlayerColor};

pub const PATH_HOUSE_BLUE: &str = "sprites/buildings/house/house_blue.png";
pub const PATH_HOUSE_RED: &str = "sprites/buildings/house/house_red.png";
pub const PATH_HOUSE_GREEN: &str = "sprites/buildings/house/house_green.png";
pub const PATH_HOUSE_ORANGE: &str = "sprites/buildings/house/house_orange.png";
pub const PATH_HOUSE_PURPLE: &str = "sprites/buildings/house/house_purple.png";
pub const PATH_HOUSE_YELLOW: &str = "sprites/buildings/house/house_yellow.png";
pub const PATH_HOUSE_AQUA: &str = "sprites/buildings/house/house_aqua.png";
pub const PATH_HOUSE_PINK: &str = "sprites/buildings/house/house_pink.png";
pub const PATH_HOUSE_NEUTRAL: &str = "sprites/buildings/house/house_neutral.png";

pub const PATH_MAIN_BUILDING_BLUE: &str = "sprites/buildings/main_building/main_building_blue.png";
pub const PATH_MAIN_BUILDING_RED: &str = "sprites/buildings/main_building/main_building_red.png";
pub const PATH_MAIN_BUILDING_GREEN: &str =
    "sprites/buildings/main_building/main_building_green.png";
pub const PATH_MAIN_BUILDING_ORANGE: &str =
    "sprites/buildings/main_building/main_building_orange.png";
pub const PATH_MAIN_BUILDING_PURPLE: &str =
    "sprites/buildings/main_building/main_building_purple.png";
pub const PATH_MAIN_BUILDING_YELLOW: &str =
    "sprites/buildings/main_building/main_building_yellow.png";
pub const PATH_MAIN_BUILDING_AQUA: &str = "sprites/buildings/main_building/main_building_aqua.png";
pub const PATH_MAIN_BUILDING_PINK: &str = "sprites/buildings/main_building/main_building_pink.png";
pub const PATH_MAIN_BUILDING_NEUTRAL: &str =
    "sprites/buildings/main_building/main_building_neutral.png";

pub fn get_path(building_type: BuildingType, player_color: PlayerColor) -> &'static str {
    match building_type {
        BuildingType::House => match player_color {
            PlayerColor::Blue => PATH_HOUSE_BLUE,
            PlayerColor::Red => PATH_HOUSE_RED,
            PlayerColor::Green => PATH_HOUSE_GREEN,
            PlayerColor::Orange => PATH_HOUSE_ORANGE,
            PlayerColor::Purple => PATH_HOUSE_PURPLE,
            PlayerColor::Yellow => PATH_HOUSE_YELLOW,
            PlayerColor::Aqua => PATH_HOUSE_AQUA,
            PlayerColor::Pink => PATH_HOUSE_PINK,
            PlayerColor::Neutral => PATH_HOUSE_NEUTRAL,
        },

        BuildingType::MainBuilding { index: _ } => match player_color {
            PlayerColor::Blue => PATH_MAIN_BUILDING_BLUE,
            PlayerColor::Red => PATH_MAIN_BUILDING_RED,
            PlayerColor::Green => PATH_MAIN_BUILDING_GREEN,
            PlayerColor::Orange => PATH_MAIN_BUILDING_ORANGE,
            PlayerColor::Purple => PATH_MAIN_BUILDING_PURPLE,
            PlayerColor::Yellow => PATH_MAIN_BUILDING_YELLOW,
            PlayerColor::Aqua => PATH_MAIN_BUILDING_AQUA,
            PlayerColor::Pink => PATH_MAIN_BUILDING_PINK,
            PlayerColor::Neutral => PATH_MAIN_BUILDING_NEUTRAL,
        },
    }
}
