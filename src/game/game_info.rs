use bevy::{platform::collections::HashMap, prelude::*};

use crate::{
    AppState,
    game::{building::BuildingId, player::PlayerRef},
};

pub mod end_game;
mod starting_decount;

pub struct GameInfoPlugin;
impl Plugin for GameInfoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((starting_decount::plugin, end_game::plugin));

        app.add_sub_state::<GameState>();
    }
}

#[derive(SubStates, Default, Hash, PartialEq, Eq, Clone, Copy, Debug)]
#[source(AppState = AppState::InGame)]
pub enum GameState {
    #[default]
    StartingDecount,
    Playing {
        paused: bool,
    },

    GameOver,
}

#[derive(Resource)]
pub struct StartGameInfo {
    pub players: Vec<Entity>,
    pub building_assignements: HashMap<BuildingId, PlayerRef>,
}
