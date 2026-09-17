use bevy::prelude::*;

use crate::{
    game::{
        bot::building_safety::BuildingSafety,
        building::{Building, building_types::BuildingType, inhabitants::Inhabitants},
        game_info::GameState,
        player::PlayerRef,
    },
    world_grid::grid_transform::GridTransform,
};

pub struct BotViewPlugin;
impl Plugin for BotViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            fill_bot_view.run_if(in_state(GameState::Playing { paused: false })),
        );
    }
}

#[derive(Component, Default, Clone)]
pub struct BotView {
    pub bot_buildings: Vec<BuildingView>,
    pub all_buildings: Vec<BuildingView>,
}

impl BotView {
    fn clear(&mut self) {
        self.bot_buildings.clear();
        self.all_buildings.clear();
    }
}

fn fill_bot_view(
    mut bots: Query<(Entity, &mut BotView)>,
    buildigns: Query<(
        Entity,
        &Building,
        Option<&PlayerRef>,
        &BuildingSafety,
        &Inhabitants,
        &GridTransform,
    )>,
) {
    for (player_entity, mut bot_view) in &mut bots {
        bot_view.clear();

        for building in &buildigns {
            let (
                building_entity,
                building,
                player_ref,
                building_safety,
                inhabitants,
                grid_transform,
            ) = building;

            let owner = match player_ref {
                Some(player_ref) => {
                    if player_ref.0 == player_entity {
                        BuildingOwner::Own
                    } else {
                        BuildingOwner::Enemy
                    }
                }
                None => BuildingOwner::Neutral,
            };

            let building_view = BuildingView {
                entity: building_entity,
                owner,
                building_type: building.building_type(),
                building_safety: *building_safety,
                inhabitants: inhabitants.current(),
                world_pos: grid_transform.center_in_world(),
            };

            bot_view.all_buildings.push(building_view.clone());

            if let Some(player_ref) = player_ref
                && player_ref.0 == player_entity
            {
                bot_view.bot_buildings.push(building_view);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingView {
    pub entity: Entity,
    pub building_type: BuildingType,
    pub owner: BuildingOwner,
    pub building_safety: BuildingSafety,
    pub inhabitants: i32,
    pub world_pos: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingOwner {
    Own,
    Enemy,
    Neutral,
}
