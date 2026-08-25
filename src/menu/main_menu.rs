use bevy::{
    feathers::{controls::FeathersButton, theme::ThemedText},
    prelude::*,
    ui_widgets::Activate,
};

use crate::{AppState, menu::MenuState};

#[derive(Component, Default, Clone)]
pub struct MainMenu;

pub fn main_menu() -> impl Scene {
    bsn! {
        MainMenu

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
        on(|_: On<Activate>, mut next_state: ResMut<NextState<MenuState>>| {
            next_state.set(MenuState::GamePreparation);
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
