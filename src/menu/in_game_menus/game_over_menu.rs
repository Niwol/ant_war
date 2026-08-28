use bevy::{
    feathers::{controls::FeathersButton, theme::ThemedText},
    prelude::*,
    ui_widgets::Activate,
};

use crate::{
    AppState,
    game::game_info::{GameState, end_game::EndGameInfo},
};

pub struct GameOverMenuPlugin;
impl Plugin for GameOverMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui);
        app.add_systems(OnExit(GameState::GameOver), despawn_game_over_ui);
    }
}

#[derive(Component, Default, Clone)]
struct GameOverMenuMarker;

fn spawn_game_over_ui(mut commands: Commands, end_game_info: Res<EndGameInfo>) {
    let winner = end_game_info.winner.unwrap();

    commands.spawn_scene(bsn! {
        GameOverMenuMarker
        Node {
            flex_direction: FlexDirection::Column,

            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,

            row_gap: px(10.0),
        }

        Children [
            (
                Text::new(
                    format!(
                        "Player {} {} won",
                        winner.team().id(),
                        winner.player_color().color_name()
                    )
                )
                TextFont {
                    font_size: px(50.0),
                }
                TextColor({winner.player_color().color()})
            ),

            (
                @FeathersButton {
                    @caption: bsn! {
                        Text::new("Restart")
                        ThemedText
                    }
                }
                on(restart_button_clicked)
            ),

            (
                @FeathersButton {
                    @caption: bsn! {
                        Text::new("Main Menu")
                        ThemedText
                    }
                }
                on(back_to_main_menu)
            ),
        ]
    });
}

fn restart_button_clicked(_: On<Activate>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::StartingDecount);
}

fn back_to_main_menu(_: On<Activate>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainMenu);
}

fn despawn_game_over_ui(
    mut commands: Commands,
    game_over_ui: Single<Entity, With<GameOverMenuMarker>>,
) {
    commands.entity(*game_over_ui).despawn();
}
