use bevy::prelude::*;

use crate::{
    game::{
        building::{Building, inhabitants::Inhabitants},
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
    pub enemy_buildings: Vec<BuildingView>,
}

impl BotView {
    fn clear(&mut self) {
        self.bot_buildings.clear();
        self.enemy_buildings.clear();
    }
}

fn fill_bot_view(
    mut bots: Query<(Entity, &mut BotView)>,
    buildigns: Query<(
        Entity,
        Option<&PlayerRef>,
        &Inhabitants,
        &Building,
        &GridTransform,
    )>,
) {
    for (player_entity, mut bot_view) in &mut bots {
        bot_view.clear();

        for building in &buildigns {
            let (building_entity, player_ref, inhabitants, building, grid_transform) = building;

            let building_view = BuildingView {
                entity: building_entity,
                inhabitants: inhabitants.total(),
                max_inhabitants: building.max_inhabitants(),
                world_pos: grid_transform.center_in_world(),
            };

            if let Some(player_ref) = player_ref
                && player_ref.0 == player_entity
            {
                bot_view.bot_buildings.push(building_view);
            } else {
                bot_view.enemy_buildings.push(building_view);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct BuildingView {
    pub entity: Entity,
    pub inhabitants: i32,
    pub max_inhabitants: i32,
    pub world_pos: Vec2,
}
