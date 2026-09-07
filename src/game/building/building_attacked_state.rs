use bevy::prelude::*;

use crate::game::{
    ant::ant_stats::AntStats,
    building::{EnterBuilding, building_stats::BuildingStats, inhabitants::Inhabitants},
    player::PlayerRef,
};

pub struct BuildingAttackedStatePlugin;
impl Plugin for BuildingAttackedStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_attacked_state);

        app.add_observer(on_ant_enters);
    }
}

#[derive(Component, Clone)]
pub struct BuildingAttackedState {
    since_last_attack: Timer,
    damage_taken: f32,
}

impl Default for BuildingAttackedState {
    fn default() -> Self {
        Self {
            since_last_attack: Timer::from_seconds(1.0, TimerMode::Once),
            damage_taken: 0.0,
        }
    }
}

fn update_attacked_state(time: Res<Time>, mut attacked_states: Query<&mut BuildingAttackedState>) {
    for mut attacked_state in &mut attacked_states {
        attacked_state.since_last_attack.tick(time.delta());

        if attacked_state.since_last_attack.is_finished() {
            attacked_state.damage_taken = 0.0;
        }
    }
}

fn on_ant_enters(
    enter_event: On<EnterBuilding>,
    mut commands: Commands,
    mut buildings: Query<(
        Entity,
        &mut BuildingAttackedState,
        &mut Inhabitants,
        &BuildingStats,
        Option<&PlayerRef>,
    )>,
    ants: Query<(Entity, &AntStats, &PlayerRef)>,
) {
    let (building_entity, mut attack_state, mut inhabitants, building_stats, building_player_ref) =
        buildings.get_mut(enter_event.building).unwrap();
    let (ant_entity, ant_stats, ant_player_ref) = ants.get(enter_event.ant).unwrap();

    commands.entity(ant_entity).despawn();

    if let Some(building_player_ref) = building_player_ref
        && building_player_ref == ant_player_ref
    {
        inhabitants.add(1);
        return;
    }

    let damage = ant_stats.attack_power / building_stats.defense;

    println!("Damage: {damage}");

    attack_state.damage_taken += damage;
    attack_state.since_last_attack.reset();

    let mut overtaken = false;

    while attack_state.damage_taken >= 1.0 {
        attack_state.damage_taken -= 1.0;

        if inhabitants.current() == 0 {
            overtaken = true;
        } else {
            inhabitants.take(1);
        }
    }

    if overtaken {
        commands.entity(building_entity).insert(*ant_player_ref);
    }
}
