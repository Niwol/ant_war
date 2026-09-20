use bevy::{
    feathers::{controls::FeathersButton, theme::ThemedText},
    prelude::*,
    ui_widgets::Activate,
};

use crate::menu::main_menu::MainMenuState;

const CONTROLS_TEXT: &str = "Controls
LMB / RMB: Left / Right Mouse Button
LMB: Select building
Left Shift + LMB: Select multiple buildings
Hold LMB + Drag: Select multiple buildings
Right Ctrl + LMB: Deselect building
RMB: Send units";

const BUILDING_TEXT: &str = "Buildings
Head Quarter: 
    - Production: 1 every 1 seconds
    - Ants produced: Workers
    - Capacity: 40
    - Defense: 1.0
House:
    - Production: 1 every 2 seconds
    - Ants produced: Units
    - Capacity: 20
    - Defense: 1.0
Casern:
    - Production: 1 every 5 seconds
    - Ants produced: Soldiers
    - Capacity: 20
    - Defense: 1.0
Tower:
    - Production: None
    - Ants produced: Units
    - Capacity: 30
    - Defense: 1.0
    - Fire radius: Large
Walls:
    - Production: None
    - Ants produced: Units
    - Capacity: 50
    - Defense: 1.5
    - Fire radius: Small
";

const ANTS_TEXT: &str = "Ants:
Worker:
    - Speed: 70px/sec
    - Attack: 0.8
    - Health: 1.0
Unit:
    - Speed: 50px/sec
    - Attack: 1.0
    - Health: 1.0
Soldier:
    - Speed: 40px/sec
    - Attack: 1.5
    - Health: 1.5
";

const COMBAT_TEXT: &str = "Combat:
When an Ant enters a building it deals damage
damage = (ant_attack + ant_health_percentage) / building_defense
When damage exeeds 1.0 the building looses 1 inhabitant
After 3 seconds without an incomming attack the building recovers 
and the remaining damage is set back to 0.0

Projectiles:
Projectiles deal 1.0 damage to ants
(a projectile typically kills ants exept soldiers that can take 2 shots)
";

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(MainMenuState::HowToPlay),
        (how_to_play_menu.spawn(), set_screen_size),
    );
}

fn set_screen_size(mut window: Single<&mut Window>) {
    window.resolution.set(1350.0, 800.0);
}

fn how_to_play_menu() -> impl Scene {
    bsn! {
        DespawnOnExit<MainMenuState>(MainMenuState::HowToPlay)

        Node {
            flex_direction: FlexDirection::Column,

            align_self: AlignSelf::Center,
            justify_self: JustifySelf::Center,

            row_gap: px(10.0)

            border: px(3.0),
            padding: px(5.0),
        }
        BorderColor::all(Color::WHITE)

        Children [
            how_to_play_panel(),

            @FeathersButton {
                @caption: bsn! {
                    Text::new("Title Screen")
                    ThemedText
                }
            }
            on(back_to_title_screen)
        ]
    }
}

fn how_to_play_panel() -> impl Scene {
    bsn! {
        Node {
            column_gap: px(5.0)
        }

        Children [
            info_panel(CONTROLS_TEXT),
            info_panel(BUILDING_TEXT),
            info_panel(ANTS_TEXT),
            info_panel(COMBAT_TEXT),
        ]
    }
}

fn info_panel(text: &'static str) -> impl Scene {
    bsn! {
        Node {
            border: px(2.0),
            border_radius: px(3.0),
            padding: px(3.0),
        }
        BorderColor::all(Color::WHITE)

        Children [
            Text::new(text)
            TextFont {
                font_size: px(12.0)
            }
        ]
    }
}

fn back_to_title_screen(_: On<Activate>, mut next_state: ResMut<NextState<MainMenuState>>) {
    next_state.set(MainMenuState::TitleScreen);
}
