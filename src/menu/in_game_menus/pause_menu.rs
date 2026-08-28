use bevy::{
    feathers::{controls::FeathersButton, theme::ThemedText},
    prelude::*,
    ui_widgets::Activate,
};

use crate::{AppState, game::game_info::GameState};

pub struct PauseMenuPlugin;
impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Playing { paused: true }),
            spawn_pause_menu,
        );
        app.add_systems(
            OnExit(GameState::Playing { paused: true }),
            despawn_pause_menu,
        );
    }
}

#[derive(Component, Default, Clone)]
struct PauseMenuMarker;

fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        PauseMenuMarker
        Node {
            flex_direction: FlexDirection::Column,

            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,

            row_gap: px(10.0),
        }

        Children [
            (
                Text::new("Pause")
                TextFont {
                    font_size: px(50.0),
                }
                TextColor(Color::srgb(0.0, 1.0, 0.0))
            ),

            (
                @FeathersButton {
                    @caption: bsn! {
                        Text::new("Resume")
                        ThemedText
                    }
                }
                on(on_resume_button_clicked)
            ),

            (
                @FeathersButton {
                    @caption: bsn! {
                        Text::new("Restart")
                        ThemedText
                    }
                }
                on(on_restart_button_clicked)
            ),

            (
                @FeathersButton {
                    @caption: bsn! {
                        Text::new("Main Menu")
                        ThemedText
                    }
                }
                on(on_main_menu_button_clicked)
            )
        ]
    });
}

fn despawn_pause_menu(mut commands: Commands, pause_menu: Single<Entity, With<PauseMenuMarker>>) {
    commands.entity(*pause_menu).despawn();
}

fn on_resume_button_clicked(_: On<Activate>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing { paused: false });
}

fn on_restart_button_clicked(_: On<Activate>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::StartingDecount);
}

fn on_main_menu_button_clicked(_: On<Activate>, mut next_state: ResMut<NextState<AppState>>) {
    next_state.set(AppState::MainMenu);
}
