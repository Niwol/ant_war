use bevy::{
    feathers::{controls::FeathersCheckbox, theme::ThemedText},
    prelude::*,
    ui_widgets::{ValueChange, checkbox_self_update},
};

use crate::AppState;

const HELPER_TEXT: &str = "Controls:
LMB / RMB: Left / Right Mouse Button
LMB: Select building
Left Shift + LMB: Select multiple buildings
Hold LMB + Drag: Select multiple buildings
Right Ctrl + LMB: Deselect building
RMB: Send units

Buildings:
Big (Head Quarter)
    - Production speed: 1 unit / sec
    - Max capacity: 40 units
Small (House):
    - Production speed: 2 units / sec
    - Max capacity: 20 units

Goal: Take over the enemy
";

pub struct ControlHelperPlugin;
impl Plugin for ControlHelperPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_control_helper_ui);
        app.add_systems(OnExit(AppState::InGame), despawn_control_helper_ui);
    }
}

#[derive(SceneComponent, Default, Clone)]
struct ControlHelperUi;

#[derive(Component, Default, Clone)]
struct HelperText;

impl ControlHelperUi {
    fn scene() -> impl Scene {
        bsn! {
            Node {
                flex_direction: FlexDirection::Column,

                align_self: AlignSelf::Center,

                border: px(2.0),
                border_radius: px(2.0),
                margin: px(5.0),
                padding: px(5.0),

                row_gap: px(3.0),
            }
            BorderColor::all(Color::WHITE)

            Children [
                @FeathersCheckbox {
                    @caption: bsn! {
                        Text::new("Help text")
                        ThemedText
                    }
                }
                on(checkbox_self_update)
                on(toggle_help_text_check_box),

                Node {
                    display: Display::None
                }
                HelperText
                Text::new(HELPER_TEXT)
                TextFont {
                    font_size: px(12.0),
                }
            ]
        }
    }
}

fn spawn_control_helper_ui(mut commands: Commands) {
    commands.spawn_scene(bsn! {
        @ControlHelperUi
    });
}

fn despawn_control_helper_ui(
    mut commands: Commands,
    helper_ui: Single<Entity, With<ControlHelperUi>>,
) {
    commands.entity(*helper_ui).despawn();
}

fn toggle_help_text_check_box(
    change: On<ValueChange<bool>>,
    mut help_text: Single<&mut Node, With<HelperText>>,
) {
    match change.value {
        true => help_text.display = Display::Flex,
        false => help_text.display = Display::None,
    }
}
