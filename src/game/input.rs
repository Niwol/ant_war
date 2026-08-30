use bevy::prelude::*;

use crate::{
    AppState,
    game::{
        InGameEntity,
        building::{
            Building,
            building_selection::{BuildingSelectionKind, DeselectAllBuildings, SelectBuilding},
        },
        game_info::GameState,
    },
};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_deselection_sprite);

        app.add_systems(Update, handle_key_input.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Event)]
pub struct InputMoveOrder {
    pub target: Entity,
}

#[derive(Component, Default, Clone, Copy)]
#[require(InGameEntity)]
struct BackgroundSprite;

fn spawn_deselection_sprite(mut commands: Commands) {
    commands
        .spawn_scene(bsn! {
            InGameEntity
            BackgroundSprite
            Sprite {
                color: Color::srgba(0.0, 0.5, 0.0, 0.0),
                custom_size: Vec2::new(3000.0, 2000.0)
            }
            Pickable
            Transform::from_xyz(0.0, 0.0, -1.0)
        })
        .observe(on_background_clicked);
}

fn on_background_clicked(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if click.button == PointerButton::Primary {
        if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ControlLeft]) {
            return;
        }

        commands.trigger(DeselectAllBuildings);
    }
}

pub fn on_select(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
) {
    if click.button == PointerButton::Primary {
        let mut select_buildign = SelectBuilding {
            building: click.entity,
            selection_kind: BuildingSelectionKind::SingleSelection,
        };

        if input.pressed(KeyCode::ShiftLeft) {
            select_buildign.selection_kind = BuildingSelectionKind::AddSelection;
        }

        if input.pressed(KeyCode::ControlLeft) {
            select_buildign.selection_kind = BuildingSelectionKind::RemoveSelection;
        }

        commands.trigger(select_buildign);
    }
}

pub fn on_order(
    click: On<Pointer<Click>>,
    buildings: Query<Entity, With<Building>>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Secondary {
        return;
    }

    if buildings.contains(click.entity) {
        commands.trigger(InputMoveOrder {
            target: click.entity,
        });
    }
}

fn handle_key_input(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match **current_state {
            GameState::StartingDecount => (),
            GameState::Playing { paused } => {
                next_state.set(GameState::Playing { paused: !paused });
            }
            GameState::GameOver => (),
        }
    }
}
