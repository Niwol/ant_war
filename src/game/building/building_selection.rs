use bevy::{color::palettes::css::WHITE, platform::collections::HashSet, prelude::*};

use crate::{
    AppState,
    game::{
        bot::Bot,
        building::Building,
        player::{Player, PlayerRef},
    },
    world_grid::grid_transform::GridTransform,
};

pub struct BuildingSelectionPlugin;
impl Plugin for BuildingSelectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), add_selected_buildings_ressource);
        app.add_systems(
            OnExit(AppState::InGame),
            remove_selected_buildings_ressource,
        );

        app.add_systems(Update, on_player_change);

        app.add_observer(on_select_building);
        app.add_observer(on_select_buildings_in_rect);
        app.add_observer(on_deselect_building);
        app.add_observer(on_deselect_all_buildings);
        app.add_observer(on_add_building_selected);
        app.add_observer(on_remove_building_selected);
    }
}

#[derive(Event)]
pub struct SelectBuilding {
    pub building: Entity,
    pub selection_kind: BuildingSelectionKind,
}

#[derive(Event)]
pub struct SelectBuildingsInRect {
    pub rect: Rect,
    pub selection_kind: BuildingSelectionKind,
}

#[derive(Event)]
pub struct DeselectBuilding {
    pub building: Entity,
}

#[derive(Event)]
pub struct DeselectAllBuildings;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BuildingSelectionKind {
    SingleSelection,
    AddSelection,
    RemoveSelection,
}

#[derive(Resource)]
pub struct SelectedBuildings {
    pub buildings: HashSet<Entity>,
}

fn add_selected_buildings_ressource(mut commands: Commands) {
    commands.insert_resource(SelectedBuildings {
        buildings: HashSet::new(),
    });
}

fn remove_selected_buildings_ressource(mut commands: Commands) {
    commands.remove_resource::<SelectedBuildings>();
}

#[derive(Component, Default, Clone)]
pub struct BuildingSelected;

#[derive(Component, Default, Clone)]
struct BuildingSelectionUi;

fn on_select_building(
    select: On<SelectBuilding>,
    mut commands: Commands,
    mut selected_buildings: ResMut<SelectedBuildings>,
    buildings: Query<Option<&PlayerRef>, With<Building>>,
    players: Query<Entity, (With<Player>, Without<Bot>)>,
) {
    match select.selection_kind {
        BuildingSelectionKind::SingleSelection => {
            for entity in &selected_buildings.buildings {
                commands.entity(*entity).remove::<BuildingSelected>();
            }
            selected_buildings.buildings.clear();

            commands.entity(select.building).insert(BuildingSelected);
            selected_buildings.buildings.insert(select.building);
        }

        BuildingSelectionKind::AddSelection => {
            if !selected_buildings.buildings.iter().all(|building| {
                let player_ref = buildings.get(*building).unwrap();
                if let Some(player_ref) = player_ref
                    && players.contains(player_ref.0)
                {
                    return true;
                }
                false
            }) {
                return;
            }

            let player_ref = buildings.get(select.building).unwrap();
            if let Some(player_ref) = player_ref
                && players.contains(player_ref.0)
            {
                commands.entity(select.building).insert(BuildingSelected);
                selected_buildings.buildings.insert(select.building);
            }
        }

        BuildingSelectionKind::RemoveSelection => {
            commands.trigger(DeselectBuilding {
                building: select.building,
            });
        }
    };
}

fn on_select_buildings_in_rect(
    select: On<SelectBuildingsInRect>,
    mut commands: Commands,
    buildings: Query<(Entity, &GridTransform)>,
) {
    let rect = select.rect;

    let selection_kind = match select.selection_kind {
        BuildingSelectionKind::SingleSelection => {
            commands.trigger(DeselectAllBuildings);
            BuildingSelectionKind::AddSelection
        }
        BuildingSelectionKind::AddSelection => BuildingSelectionKind::AddSelection,
        BuildingSelectionKind::RemoveSelection => BuildingSelectionKind::RemoveSelection,
    };

    for (entity, grid_transform) in &buildings {
        let building_rect = grid_transform.rect_in_world();
        if !building_rect.intersect(rect).is_empty() {
            commands.trigger(SelectBuilding {
                building: entity,
                selection_kind: selection_kind,
            });
        }
    }
}

fn on_deselect_all_buildings(
    _: On<DeselectAllBuildings>,
    mut commands: Commands,
    selected_buildings: Res<SelectedBuildings>,
) {
    for building in &selected_buildings.buildings {
        commands.trigger(DeselectBuilding {
            building: *building,
        });
    }
}

fn on_deselect_building(
    deselect: On<DeselectBuilding>,
    mut commands: Commands,
    mut selected_buildings: ResMut<SelectedBuildings>,
) {
    if selected_buildings.buildings.contains(&deselect.building) {
        selected_buildings.buildings.remove(&deselect.building);
        commands
            .entity(deselect.building)
            .remove::<BuildingSelected>();
    }
}

fn on_player_change(
    mut commands: Commands,
    buildings: Query<Entity, (With<Building>, Changed<PlayerRef>)>,
) {
    for building in buildings {
        commands.trigger(DeselectBuilding { building });
    }
}

fn on_add_building_selected(
    add: On<Add, BuildingSelected>,
    mut commands: Commands,
    sprites: Query<&Sprite, With<Building>>,
    images: Res<Assets<Image>>,
) {
    let sprite = sprites.get(add.entity).unwrap();

    let sprite_size = if let Some(custom_size) = sprite.custom_size {
        custom_size
    } else if let Some(image) = images.get(&sprite.image) {
        image.size_f32()
    } else {
        warn!("Building somehow does not have a sprite or a custom size");
        Vec2::ONE
    };

    let selection_ui = commands
        .spawn_scene(bsn! {
            BuildingSelectionUi
            Sprite {
                custom_size: {Some(Vec2 { x: sprite_size.x, y: 2.0 })},
                color: WHITE
            }
            Transform::from_xyz(0.0, -(sprite_size.y / 2.0 + 5.0), 0.0)
        })
        .id();

    commands.entity(add.entity).add_child(selection_ui);
}

fn on_remove_building_selected(
    remove: On<Remove, BuildingSelected>,
    mut commands: Commands,
    selection_uis: Query<(Entity, &ChildOf), With<BuildingSelectionUi>>,
) {
    for (entity, child_of) in &selection_uis {
        if child_of.0 == remove.entity {
            commands.entity(entity).try_despawn();
        }
    }
}
