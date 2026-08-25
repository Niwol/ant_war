use bevy::prelude::*;

use crate::game::{
    building::{Building, inhabitants::Inhabitants},
    game_info::GameState,
    input::MoveOrder,
    team::PlayerRef,
};

pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_bots.run_if(in_state(GameState::Playing { paused: false })),
        );
    }
}

#[derive(Component, Default, Clone, Copy)]
pub struct Bot;

fn update_bots(
    mut commands: Commands,
    bots: Query<Entity, With<Bot>>,
    player_buildings: Query<(Entity, &PlayerRef, &Inhabitants), With<Building>>,
    all_buildings: Query<(Entity, Option<&PlayerRef>), With<Building>>,
) {
    for bot in &bots {
        for (building_entity, building_ref, inhabitants) in &player_buildings {
            if bot == building_ref.0 && inhabitants.total() >= 15 {
                for (other_building_entity, other_building_ref) in &all_buildings {
                    if let Some(other_building_ref) = other_building_ref
                        && other_building_ref.0 == bot
                    {
                        continue;
                    }

                    commands.trigger(MoveOrder {
                        entity: building_entity,
                        target: other_building_entity,
                    });
                    break;
                }
            }
        }
    }
}
