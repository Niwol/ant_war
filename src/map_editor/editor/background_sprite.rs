use bevy::prelude::*;

use crate::{
    game::building::building_type::BuildingType,
    map::BuildingInfo,
    map_editor::{
        MapEditorEntity,
        editor::{
            CurrentMap, EditorState,
            editor_building::{DeselectBuilding, SpawnEditorBuilding},
            preview_building::PreviewBuilding,
        },
    },
    world_grid::{WorldGrid, grid_transform::GridTransform},
};

#[derive(SceneComponent, Default, Clone)]
#[require(MapEditorEntity)]
#[scene(BackgroundSpriteProps)]
pub struct BackgroundSprite;

#[derive(Default, Clone)]
pub struct BackgroundSpriteProps {
    pub size: Vec2,
    pub pos: Vec2,
}

impl BackgroundSprite {
    fn scene(props: BackgroundSpriteProps) -> impl Scene {
        bsn! {
            Sprite {
                color: Color::srgba(0.0, 1.0, 0.0, 0.3),
                custom_size: {
                    Some(props.size)
                },
            }
            Transform::from_xyz(props.pos.x, props.pos.y, -1.0)
            Pickable
            on(on_background_left_click)
            on(on_background_right_click)
            on(show_preview_building)
            on(hide_preview_building)

        }
    }
}

fn on_background_left_click(
    click: On<Pointer<Click>>,
    input: Res<ButtonInput<KeyCode>>,
    world_grid: Res<WorldGrid>,
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    mut editor_state: ResMut<EditorState>,
    mut preview_building: Query<(&mut PreviewBuilding, &GridTransform)>,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    if let Some(entity) = editor_state.selected_building {
        commands.trigger(DeselectBuilding { entity });
        editor_state.selected_building = None;
    }

    if let Some(preview_building_entity) = editor_state.add_building_preveiw {
        let (mut preview_building, grid_transform) =
            preview_building.get_mut(preview_building_entity).unwrap();

        let building_info = BuildingInfo {
            building_type: preview_building.building_type,
            grid_transform: *grid_transform,
        };

        if world_grid.all_cells_empty(&building_info.grid_transform.get_coords()) {
            let building_id = current_map.map.add_building(building_info);

            commands.trigger(SpawnEditorBuilding {
                building_id,
                building_info,
            });

            match preview_building.building_type {
                BuildingType::House => (),
                BuildingType::MainBuilding { index: _ } => {
                    let new_index = current_map.map.next_main_building_index();
                    preview_building.building_type = BuildingType::MainBuilding { index: new_index }
                }
                BuildingType::Tower => (),
            }

            if !input.pressed(KeyCode::ShiftLeft) {
                commands.entity(preview_building_entity).despawn();
                editor_state.add_building_preveiw = None;
            }
        }
    }
}

fn on_background_right_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut editor_state: ResMut<EditorState>,
) {
    if click.button != PointerButton::Secondary {
        return;
    }

    if let Some(entity) = editor_state.selected_building {
        commands.trigger(DeselectBuilding { entity });
        editor_state.selected_building = None;
    }

    if let Some(preview_building_entity) = editor_state.add_building_preveiw {
        commands.entity(preview_building_entity).despawn();
        editor_state.add_building_preveiw = None;
    }
}

fn show_preview_building(
    _: On<Pointer<Enter>>,
    preview_building_visibility: Single<&mut Visibility, With<PreviewBuilding>>,
) {
    let mut visibility = preview_building_visibility.into_inner();
    *visibility = Visibility::Inherited;
}

fn hide_preview_building(
    _: On<Pointer<Leave>>,
    preview_building_visibility: Single<&mut Visibility, With<PreviewBuilding>>,
) {
    let mut visibility = preview_building_visibility.into_inner();
    *visibility = Visibility::Hidden;
}
