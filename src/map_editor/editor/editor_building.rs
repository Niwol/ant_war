use std::f32::consts::PI;

use bevy::prelude::*;

use crate::{
    game::{
        building::{self, BuildingId, BuildingType},
        player::PlayerColor,
    },
    map::BuildingInfo,
    map_editor::{
        MapEditorEntity,
        editor::{CurrentMap, DragBuilding, EditorState, invalid_location::InvalidLocation},
    },
    world_grid::{WorldGrid, cell::Cell, grid_transform::GridTransform},
};
pub struct EditorBuildingPlugin;
impl Plugin for EditorBuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_building_texts);

        app.add_observer(spawn_building);
    }
}

#[derive(EntityEvent)]
pub struct DeselectBuilding {
    pub entity: Entity,
}

#[derive(Event)]
pub struct SpawnEditorBuilding {
    pub building_id: BuildingId,
    pub building_info: BuildingInfo,
}

#[derive(SceneComponent, Default, Clone)]
#[require(Pickable, MapEditorEntity)]
#[scene(EditorBuildingProps)]
pub struct EditorBuilding {
    building_type: BuildingType,
}

#[derive(Default)]
pub struct EditorBuildingProps {
    pub building_id: BuildingId,
    pub building_type: BuildingType,
    pub player_color: PlayerColor,
    pub grid_transfrom: GridTransform,
}

#[derive(Component, Default, Clone)]
struct BuildingText;

impl EditorBuilding {
    fn scene(props: EditorBuildingProps) -> impl Scene {
        let building_path =
            building::asset_paths::get_path(props.building_type, props.player_color);

        let building_text = match props.building_type {
            BuildingType::House => format!("House"),
            BuildingType::MainBuilding { index } => format!("HQ {index}"),
            BuildingType::Tower => format!("Tower"),
        };

        bsn! {
            EditorBuilding {
                building_type: {props.building_type},
            }

            template_value(props.building_id)

            Sprite {
                image: building_path
            }

            template_value(props.grid_transfrom)

            on(select_building)
            on(building_exit_selected_state)
            on(despawn_editor_building)

            Children [
                BuildingText
                Text2d::new(building_text)
                TextFont {
                    font_size: px(15.0)
                }
            ]
        }
    }

    pub fn _building_type(&self) -> BuildingType {
        self.building_type
    }
}

fn spawn_building(
    spawn: On<SpawnEditorBuilding>,
    mut commands: Commands,
    mut world_grid: ResMut<WorldGrid>,
) {
    let SpawnEditorBuilding {
        building_id,
        building_info,
    } = *spawn;

    let entity = commands
        .spawn_scene(bsn! {
            @EditorBuilding {
                @building_id: {building_id},
                @building_type: {building_info.building_type},
                @player_color: PlayerColor::Neutral,
                @grid_transfrom: {building_info.grid_transform},
            }
        })
        .id();

    for coord in building_info.grid_transform.get_coords() {
        world_grid.set_cell(coord, Cell::Occupied { entity });
    }
}

fn despawn_editor_building(
    despawn: On<DespawnEditorBuilding>,
    mut commands: Commands,
    mut current_map: ResMut<CurrentMap>,
    editor_buildings: Query<(&GridTransform, &BuildingId)>,
    mut world_grid: ResMut<WorldGrid>,
) {
    let (grid_transform, building_id) = editor_buildings.get(despawn.entity).unwrap();

    world_grid.set_all_cells(&grid_transform.get_coords(), Cell::Empty);

    current_map.map.remove_building(*building_id);

    commands.entity(despawn.entity).despawn();
}

#[derive(Component, Default, Clone)]
struct SelectingEntity;

#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
pub struct DespawnEditorBuilding {
    pub entity: Entity,
}

#[derive(SceneComponent, Default, Clone)]
#[scene(CrossSpriteProps)]
struct CrossSprite;

#[derive(Default)]
struct CrossSpriteProps {
    position_offset: Vec2,
}

impl CrossSprite {
    fn scene(props: CrossSpriteProps) -> impl Scene {
        bsn! {
            SelectingEntity
            Sprite {
                image: "icons/cross.png",
            }
            Transform::from_xyz(props.position_offset.x, props.position_offset.y, 0.0)
            Pickable
            on(|click: On<Pointer<Click>>, mut commands: Commands|
                commands.trigger(DespawnEditorBuilding { entity: click.entity })
            )
        }
    }
}

fn select_building(
    click: On<Pointer<Click>>,
    mut commands: Commands,
    images: Res<Assets<Image>>,
    sprites: Query<&Sprite, With<EditorBuilding>>,
    mut editor_state: ResMut<EditorState>,
) {
    if let Some(entity) = editor_state.selected_building {
        if click.entity == entity {
            return;
        }
    }

    let sprite = sprites.get(click.entity).unwrap();
    let image = images.get(&sprite.image).unwrap();

    let image_size = image.size();
    let x = (image_size.x / 2) as f32 + 10.0;
    let y = (image_size.y / 2) as f32 + 10.0;

    let cross_icon = commands
        .spawn_scene(bsn! {
            @CrossSprite {
                @position_offset: Vec2 { x, y }
            }
        })
        .id();

    let move_arrows = (0..4)
        .into_iter()
        .map(|i| {
            let rotation = (-PI / 2.0) * i as f32;
            let image_size = image.size();

            let x = (image_size.x / 2) as f32 + 10.0;
            let y = (image_size.y / 2) as f32 + 10.0;

            let (x, y) = match i {
                0 => (0.0, y),
                1 => (x, 0.0),
                2 => (0.0, -y),
                3 => (-x, 0.0),

                _ => unreachable!("End of loop"),
            };

            let mut transform = Transform::from_rotation(Quat::from_rotation_z(rotation));
            transform.translation = Vec3::new(x, y, 0.0);

            commands
                .spawn_scene(bsn! {
                    SelectingEntity
                    Sprite {
                        image: "icons/move_arrow.png",
                    }
                    template_value(transform)
                })
                .id()
        })
        .collect::<Vec<_>>();

    let drag_sprite = commands
        .spawn_scene(bsn! {
            SelectingEntity
            Sprite {
                color: Color::srgba(0.0, 1.0, 1.0, 0.2),
                custom_size: {
                    Some(image_size.as_vec2())
                }
            }
            Transform::from_xyz(0.0, 0.0, 1.0)
            Pickable
            on(start_drag_building)
            on(end_drag_building)
            on(drag_building)
        })
        .id();

    commands.entity(click.entity).add_child(cross_icon);
    commands.entity(click.entity).add_children(&move_arrows);
    commands.entity(click.entity).add_child(drag_sprite);

    if let Some(entity) = editor_state.selected_building {
        commands.trigger(DeselectBuilding { entity });
    }

    editor_state.selected_building = Some(click.entity);
}

fn building_exit_selected_state(
    deselect: On<DeselectBuilding>,
    mut commands: Commands,
    selecting_entities: Query<(Entity, &ChildOf), With<SelectingEntity>>,
) {
    for (entity, child_of) in &selecting_entities {
        if child_of.0 == deselect.entity {
            commands.entity(entity).despawn();
        }
    }
}

fn start_drag_building(
    drag_start: On<Pointer<DragStart>>,
    mut commands: Commands,
    children: Query<&ChildOf>,
    grid_transforms: Query<&GridTransform, With<EditorBuilding>>,
    mut world_grid: ResMut<WorldGrid>,
) {
    let building_entity = children.get(drag_start.entity).unwrap().0;
    let origin_transform = *grid_transforms.get(building_entity).unwrap();
    commands
        .entity(building_entity)
        .insert(DragBuilding { origin_transform });

    world_grid.set_all_cells(&origin_transform.get_coords(), Cell::Empty);
}

fn end_drag_building(
    drag_end: On<Pointer<DragEnd>>,
    mut commands: Commands,
    mut curretn_map: ResMut<CurrentMap>,
    children: Query<&ChildOf>,
    mut editor_buildings: Query<(
        &mut GridTransform,
        &BuildingId,
        &DragBuilding,
        Option<&InvalidLocation>,
    )>,
    mut world_grid: ResMut<WorldGrid>,
) {
    let building_entity = children.get(drag_end.entity).unwrap().0;
    commands.entity(building_entity).remove::<DragBuilding>();

    let (mut grid_transform, building_id, drag_building, invalid_location) =
        editor_buildings.get_mut(building_entity).unwrap();

    if invalid_location.is_some() {
        *grid_transform = drag_building.origin_transform;
        commands.entity(building_entity).remove::<InvalidLocation>();
    }

    let building_info = curretn_map.map.get_building_info_mut(*building_id).unwrap();
    building_info.grid_transform = *grid_transform;

    world_grid.set_all_cells(
        &grid_transform.get_coords(),
        Cell::Occupied {
            entity: building_entity,
        },
    );
}

fn drag_building(
    drag: On<Pointer<Drag>>,
    mut commands: Commands,
    children: Query<&ChildOf>,
    mut buildings: Query<(&mut GridTransform, &DragBuilding)>,
    world_grid: Res<WorldGrid>,
) {
    let building_entity = children.get(drag.entity).unwrap().0;
    let (mut building_transform, drag_building) = buildings.get_mut(building_entity).unwrap();

    let origin_center = drag_building.origin_transform.center_in_world();
    let new_center = origin_center
        + Vec2 {
            x: drag.distance.x,
            y: -drag.distance.y,
        };

    building_transform.update_from_world(new_center);

    if world_grid.all_cells_empty(&building_transform.get_coords())
        && world_grid.transform_in_grid(*building_transform)
    {
        commands.entity(building_entity).remove::<InvalidLocation>();
    } else {
        commands.entity(building_entity).insert(InvalidLocation);
    }
}

fn update_building_texts(
    buildings: Query<(Entity, &EditorBuilding), Changed<EditorBuilding>>,
    mut building_texts: Query<(&ChildOf, &mut Text2d)>,
) {
    for (entity, editor_building) in &buildings {
        for (child_of, mut text) in &mut building_texts {
            if entity == child_of.0 {
                text.0 = match editor_building.building_type {
                    BuildingType::House => format!("House"),
                    BuildingType::MainBuilding { index } => format!("HQ {index}"),
                    BuildingType::Tower => format!("Tower"),
                }
            }
        }
    }
}
