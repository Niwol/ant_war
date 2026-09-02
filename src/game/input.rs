use bevy::prelude::*;

use crate::{
    AppState,
    cursor::Cursor,
    game::{
        InGameEntity,
        building::{
            Building,
            building_selection::{
                BuildingSelectionKind, DeselectAllBuildings, SelectBuilding, SelectBuildingsInRect,
            },
        },
        game_info::GameState,
    },
    ui::selection_rect::{SelectionRect, StartSelectionRect},
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
            BackgroundSprite
            Sprite {
                color: Color::srgba(0.0, 0.5, 0.0, 0.0),
                custom_size: Vec2::new(3000.0, 2000.0)
            }
            Pickable
            Transform::from_xyz(0.0, 0.0, -1.0)
        })
        .observe(on_background_clicked)
        .observe(start_selection_rect)
        .observe(update_selection_rect)
        .observe(end_selection_rect);
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

fn start_selection_rect(drag: On<Pointer<DragStart>>, cursor: Res<Cursor>, mut commands: Commands) {
    if drag.button != PointerButton::Primary {
        return;
    }

    let world_pos = cursor.world_pos();
    if let Some(world_pos) = world_pos {
        commands.trigger(StartSelectionRect {
            start_position: world_pos,
        });
    }
}

fn update_selection_rect(drag: On<Pointer<Drag>>, mut selection_rect: Single<&mut SelectionRect>) {
    let size = Vec2 {
        x: drag.distance.x,
        y: -drag.distance.y,
    };
    selection_rect.size = size;
}

fn end_selection_rect(
    _: On<Pointer<DragEnd>>,
    mut commands: Commands,
    input: Res<ButtonInput<KeyCode>>,
    selection_rect: Single<(Entity, &SelectionRect)>,
) {
    let (entity, selection_rect) = selection_rect.into_inner();
    let center = selection_rect.start_position + selection_rect.size / 2.0;

    let mut select = SelectBuildingsInRect {
        rect: Rect::from_center_size(center, selection_rect.size.abs()),
        selection_kind: BuildingSelectionKind::AddSelection,
    };

    if input.pressed(KeyCode::ShiftLeft) {
        select.selection_kind = BuildingSelectionKind::AddSelection;
    }

    if input.pressed(KeyCode::ControlLeft) {
        select.selection_kind = BuildingSelectionKind::RemoveSelection;
    }

    commands.trigger(select);

    commands.entity(entity).despawn();
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
    current_state: Res<State<GameState>>,
) {
    if click.button != PointerButton::Secondary {
        return;
    }

    if **current_state != (GameState::Playing { paused: false }) {
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
