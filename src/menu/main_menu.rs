use bevy::{
    feathers::{controls::FeathersButton, theme::ThemedText},
    prelude::*,
    ui_widgets::Activate,
};

use crate::AppState;

mod how_to_play_menu;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(how_to_play_menu::plugin);

        app.add_sub_state::<MainMenuState>();

        app.add_systems(OnEnter(MainMenuState::TitleScreen), spawn_main_menu);
    }
}

#[derive(SubStates, Hash, Debug, PartialEq, Eq, Default, Clone, Copy)]
#[source(AppState = AppState::MainMenu)]
pub enum MainMenuState {
    #[default]
    TitleScreen,
    GamePreparation,
    HowToPlay,
}

fn spawn_main_menu(mut commands: Commands, mut window: Single<&mut Window>) {
    window.resolution.set(1000.0, 800.0);
    commands.spawn_scene(title_screen());
}

pub fn title_screen() -> impl Scene {
    bsn! {
        DespawnOnExit<MainMenuState>(MainMenuState::TitleScreen)

        Node {
            flex_direction: FlexDirection::Column,

            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,

            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,

            row_gap: px(20.0),
        }

        Children [
            title_text(),
            buttons(),
        ]
    }
}

fn title_text() -> impl Scene {
    bsn! {
        Text::new("Ant War")
        TextColor(Color::srgb(0.0, 1.0, 0.0))
        TextFont {
            font_size: { 80.0 }
        }
    }
}

fn buttons() -> impl Scene {
    bsn! {
        Node {
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,

            row_gap: px(5.0)
        }

        Children [
            play_button(),
            how_to_play_button(),
            map_editor_button(),
            exit_button(),
        ]
    }
}

fn play_button() -> impl Scene {
    bsn! {
        @FeathersButton {
            @caption: bsn! {
                Text::new("Play")
                ThemedText
            }
        }
        on(|_: On<Activate>, mut next_state: ResMut<NextState<MainMenuState>>| {
            next_state.set(MainMenuState::GamePreparation);
        })
    }
}

fn how_to_play_button() -> impl Scene {
    bsn! {
        @FeathersButton {
            @caption: bsn! {
                Text::new("How To Play")
                ThemedText
            }
        }
        on(|_: On<Activate>, mut next_state: ResMut<NextState<MainMenuState>>| {
            next_state.set(MainMenuState::HowToPlay);
        })
    }
}

fn map_editor_button() -> impl Scene {
    bsn! {
        @FeathersButton {
            @caption: bsn! {
                Text::new("Map editor")
                ThemedText
            }
        }
        on(|_: On<Activate>, mut next_state: ResMut<NextState<AppState>>| {
            next_state.set(AppState::InMapEditor);
        })
    }
}

fn exit_button() -> impl Scene {
    #[cfg(not(target_family = "wasm"))]
    bsn! {
        @FeathersButton {
            @caption: bsn! {
                Text::new("Exit")
                ThemedText
            }
        }
        on(|_: On<Activate>, mut message_writer: MessageWriter<AppExit>| {
            message_writer.write(AppExit::Success);
        })
    }
}
