use bevy::{platform::collections::HashSet, prelude::*};

use crate::{
    AppState,
    game::{
        game_info::GameState,
        player::{Player, PlayerRef},
    },
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), add_end_game_info);
    app.add_systems(OnExit(AppState::InGame), remove_end_game_info);

    app.add_systems(
        Update,
        (update_game_end_info, end_game).run_if(in_state(GameState::Playing { paused: false })),
    );
}

#[derive(Resource)]
pub struct EndGameInfo {
    pub winner: Option<Player>,
}

fn add_end_game_info(mut commands: Commands) {
    commands.insert_resource(EndGameInfo { winner: None });
}

fn remove_end_game_info(mut commands: Commands) {
    commands.remove_resource::<EndGameInfo>();
}

fn update_game_end_info(
    mut end_game_info: ResMut<EndGameInfo>,
    players: Query<&Player>,
    player_refs: Query<&PlayerRef>,
) {
    let mut players_alive = HashSet::new();

    for player_ref in &player_refs {
        players_alive.insert(player_ref.0);
    }

    if players_alive.len() == 1 {
        let player_entity = players_alive.iter().next().unwrap();
        let player = players.get(*player_entity).unwrap();

        end_game_info.winner = Some(*player);
    }
}

fn end_game(end_game_info: Res<EndGameInfo>, mut next_state: ResMut<NextState<GameState>>) {
    if end_game_info.winner.is_some() {
        next_state.set(GameState::GameOver);
    }
}
