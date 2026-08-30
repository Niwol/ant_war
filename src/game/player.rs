use bevy::prelude::*;

use crate::game::{InGameEntity, bot::Bot};

pub const PLAYER_COLOR_BLUE: Color = Color::srgb(0.00, 0.50, 1.00);
pub const PLAYER_COLOR_RED: Color = Color::srgb(1.00, 0.00, 0.00);
pub const PLAYER_COLOR_GREEN: Color = Color::srgb(0.00, 1.00, 0.00);
pub const PLAYER_COLOR_ORANGE: Color = Color::srgb(1.00, 0.50, 0.00);
pub const PLAYER_COLOR_PURPLE: Color = Color::srgb(0.80, 0.00, 0.80);
pub const PLAYER_COLOR_YELLOW: Color = Color::srgb(1.00, 1.00, 0.00);
pub const PLAYER_COLOR_AQUA: Color = Color::srgb(0.00, 1.00, 1.00);
pub const PLAYER_COLOR_PINK: Color = Color::srgb(1.00, 0.50, 1.00);

pub const PLAYER_COLOR_NEUTRAL: Color = Color::srgb(0.7, 0.7, 0.7);

pub const PLAYER_COLOR_LIST: [Color; NB_PLAYER_COLORS] = [
    PLAYER_COLOR_BLUE,
    PLAYER_COLOR_RED,
    PLAYER_COLOR_GREEN,
    PLAYER_COLOR_ORANGE,
    PLAYER_COLOR_PURPLE,
    PLAYER_COLOR_YELLOW,
    PLAYER_COLOR_AQUA,
    PLAYER_COLOR_PINK,
];

pub const NB_PLAYER_COLORS: usize = 8;

pub fn _team_color_index(color: Color) -> usize {
    for i in 0..NB_PLAYER_COLORS {
        if color == PLAYER_COLOR_LIST[i] {
            return i;
        }
    }

    panic!("Color is not a TEAM COLOR");
}

#[derive(PartialEq, Eq, Clone, Copy, Hash, Default)]
pub enum PlayerColor {
    Blue,
    Red,
    Green,
    Orange,
    Purple,
    Yellow,
    Aqua,
    Pink,

    #[default]
    Neutral,
}

impl From<Color> for PlayerColor {
    fn from(value: Color) -> Self {
        match value {
            PLAYER_COLOR_BLUE => Self::Blue,
            PLAYER_COLOR_RED => Self::Red,
            PLAYER_COLOR_GREEN => Self::Green,
            PLAYER_COLOR_ORANGE => Self::Orange,
            PLAYER_COLOR_PURPLE => Self::Purple,
            PLAYER_COLOR_YELLOW => Self::Yellow,
            PLAYER_COLOR_AQUA => Self::Aqua,
            PLAYER_COLOR_PINK => Self::Pink,

            PLAYER_COLOR_NEUTRAL => Self::Neutral,

            _ => panic!("Wrong player color"),
        }
    }
}

impl PlayerColor {
    pub fn color(&self) -> Color {
        match self {
            PlayerColor::Blue => PLAYER_COLOR_BLUE,
            PlayerColor::Red => PLAYER_COLOR_RED,
            PlayerColor::Green => PLAYER_COLOR_GREEN,
            PlayerColor::Orange => PLAYER_COLOR_ORANGE,
            PlayerColor::Purple => PLAYER_COLOR_PURPLE,
            PlayerColor::Yellow => PLAYER_COLOR_YELLOW,
            PlayerColor::Aqua => PLAYER_COLOR_AQUA,
            PlayerColor::Pink => PLAYER_COLOR_PINK,

            PlayerColor::Neutral => PLAYER_COLOR_NEUTRAL,
        }
    }

    pub fn color_name(&self) -> &'static str {
        match self {
            Self::Blue => "Blue",
            Self::Red => "Red",
            Self::Green => "Green",
            Self::Orange => "Orange",
            Self::Purple => "Purple",
            Self::Yellow => "Yellow",
            Self::Aqua => "Aqua",
            Self::Pink => "Pink",

            Self::Neutral => "Neutral",
        }
    }

    pub fn next_color(&self) -> Self {
        match self {
            PlayerColor::Blue => PlayerColor::Red,
            PlayerColor::Red => PlayerColor::Green,
            PlayerColor::Green => PlayerColor::Orange,
            PlayerColor::Orange => PlayerColor::Purple,
            PlayerColor::Purple => PlayerColor::Yellow,
            PlayerColor::Yellow => PlayerColor::Aqua,
            PlayerColor::Aqua => PlayerColor::Pink,
            PlayerColor::Pink => PlayerColor::Blue,

            // Neutral always goes to first color => Blue
            PlayerColor::Neutral => PlayerColor::Blue,
        }
    }

    pub fn previous_color(&self) -> Self {
        match self {
            PlayerColor::Blue => PlayerColor::Pink,
            PlayerColor::Red => PlayerColor::Blue,
            PlayerColor::Green => PlayerColor::Red,
            PlayerColor::Orange => PlayerColor::Green,
            PlayerColor::Purple => PlayerColor::Orange,
            PlayerColor::Yellow => PlayerColor::Purple,
            PlayerColor::Aqua => PlayerColor::Yellow,
            PlayerColor::Pink => PlayerColor::Aqua,

            // Neutral always goes to first color => Blue
            PlayerColor::Neutral => PlayerColor::Blue,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub struct Team(u32);

impl Team {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, FromTemplate)]
pub struct PlayerRef(pub Entity);

#[derive(SceneComponent, Default, Clone, Copy, PartialEq)]
#[require(InGameEntity)]
#[scene(PlayerProps)]
pub struct Player {
    pub team: Team,
    pub player_color: PlayerColor,
}

#[derive(Default)]
pub struct PlayerProps {
    pub bot: Option<Bot>,
}

impl Player {
    fn scene(props: PlayerProps) -> impl Scene {
        let bot: Box<dyn Scene> = match props.bot {
            Some(bot) => Box::new(bsn! {template_value(bot)}),
            None => Box::new(bsn! {}),
        };

        bsn! {
            {bot}
        }
    }
}
