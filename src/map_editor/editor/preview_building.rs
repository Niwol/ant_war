use bevy::prelude::*;

use crate::{
    cursor::Cursor,
    game::{
        building::{self, building_type::BuildingType},
        player::PlayerColor,
    },
    map_editor::{
        MapEditorEntity, MapEditorState,
        editor::{
            AddBuilding, EditorState, editor_building::DeselectBuilding,
            invalid_location::InvalidLocation,
        },
    },
    world_grid::{WorldGrid, grid_transform::GridTransform},
};

pub struct PreviewBuildingPlugin;
impl Plugin for PreviewBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_preview_building_transform.run_if(in_state(MapEditorState::InEditor)),
        );

        app.add_observer(on_add_building);
    }
}

#[derive(SceneComponent, Default, Clone)]
#[require(MapEditorEntity)]
#[scene(PreviewBuildingProps)]
pub struct PreviewBuilding {
    pub building_type: BuildingType,
}

#[derive(Default)]
pub struct PreviewBuildingProps {
    pub building_type: BuildingType,
}

impl PreviewBuilding {
    fn scene(props: PreviewBuildingProps) -> impl Scene {
        bsn! {
            PreviewBuilding {
                building_type: {props.building_type}
            }

            Sprite {
                image: building::asset_paths::get_path(props.building_type, PlayerColor::Neutral)
                color: Color::srgba(1.0, 1.0, 1.0, 0.5),
            }

            GridTransform::new(props.building_type.grid_size())

            Visibility::Hidden
        }
    }
}

fn on_add_building(
    add: On<AddBuilding>,
    mut commands: Commands,
    preview_building: Query<Entity, With<PreviewBuilding>>,
    mut editor_state: ResMut<EditorState>,
) {
    for entity in &preview_building {
        commands.entity(entity).despawn();
    }

    let building = commands
        .spawn_scene(bsn! {
            @PreviewBuilding {
                @building_type: {add.0},
            }
        })
        .id();

    editor_state.add_building_preveiw = Some(building);

    if let Some(entity) = editor_state.selected_building {
        commands.trigger(DeselectBuilding { entity });
        editor_state.selected_building = None;
    }
}

fn update_preview_building_transform(
    mut commands: Commands,
    preview_building: Single<(Entity, &mut GridTransform), With<PreviewBuilding>>,
    cursor: Res<Cursor>,
    world_grid: Res<WorldGrid>,
) {
    let Some(world) = cursor.world_pos() else {
        return;
    };

    let (entity, mut grid_transform) = preview_building.into_inner();
    grid_transform.update_from_world(world);

    if world_grid.all_cells_empty(&grid_transform.get_coords())
        && world_grid.transform_in_grid(*grid_transform)
    {
        commands.entity(entity).remove::<InvalidLocation>();
    } else {
        commands.entity(entity).insert(InvalidLocation);
    }
}
