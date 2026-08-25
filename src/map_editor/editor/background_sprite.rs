use bevy::prelude::*;

use crate::{
    map_editor::{
        MapEditorEntity,
        editor::{
            EditorState,
            editor_building::{DeselectBuilding, SpawnEditorBuilding},
            preview_building::PreviewBuilding,
        },
    },
    world_grid::grid_transform::GridTransform,
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
            on(show_preview_building)
            on(hide_preview_building)

        }
    }
}

fn on_background_left_click(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    mut editor_state: ResMut<EditorState>,
    preview_building: Query<(&PreviewBuilding, &GridTransform)>,
) {
    if click.button != PointerButton::Primary {
        return;
    }

    if let Some(entity) = editor_state.selected_building {
        commands.trigger(DeselectBuilding { entity });
        editor_state.selected_building = None;
    }

    if let Some(entity) = editor_state.add_building_preveiw {
        let (preview_building, grid_transform) = preview_building.get(entity).unwrap();

        commands.trigger(SpawnEditorBuilding {
            building_type: preview_building.building_type,
            grid_transfrom: *grid_transform,
        });

        commands.entity(entity).despawn();
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
