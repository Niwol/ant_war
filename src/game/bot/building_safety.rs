use bevy::prelude::*;

use crate::{
    AppState,
    game::{
        ant::{AntTarget, ant_stats::AntStats},
        building::{
            Building, building_selection::BuildingSelected, building_stats::BuildingStats,
            inhabitants::Inhabitants,
        },
        player::PlayerRef,
    },
    world_grid::grid_transform::GridTransform,
};

pub struct BuildingSafetyPlugin;
impl Plugin for BuildingSafetyPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_building);
        app.add_systems(
            PreUpdate,
            (
                reset_building_safeties,
                (
                    update_defense_safety,
                    update_spatial_safety,
                    update_incomming_safety,
                ),
            )
                .chain()
                .run_if(in_state(AppState::InGame)),
        );
        // app.add_systems(
        //     PreUpdate,
        //     print_building_safety.run_if(in_state(AppState::InGame)),
        // );
    }
}

#[derive(Component, Debug, Default, Clone, Copy)]
pub struct BuildingSafety {
    pub spatial: f32,
    pub incomming: f32,
    pub defense: f32,
}

impl BuildingSafety {
    pub fn incomming_damage(&self) -> f32 {
        -self.defense - self.incomming
    }

    pub fn _total_safety(&self) -> f32 {
        self.spatial + self.incomming + self.defense
    }
}

fn reset_building_safeties(mut building_safeties: Query<&mut BuildingSafety>) {
    for mut safety in &mut building_safeties {
        *safety = BuildingSafety::default();
    }
}

fn on_add_building(add: On<Add, Building>, mut commands: Commands) {
    commands
        .entity(add.entity)
        .insert(BuildingSafety::default());
}

fn update_defense_safety(
    mut building_safeties: Query<(&mut BuildingSafety, &BuildingStats, &Inhabitants)>,
) {
    for (mut safety, stats, inhabitants) in &mut building_safeties {
        let defense_strength = stats.defense * inhabitants.current() as f32;
        safety.defense = defense_strength;
    }
}

fn update_spatial_safety(
    mut building_safeties: Query<(
        Entity,
        &mut BuildingSafety,
        &GridTransform,
        Option<&PlayerRef>,
    )>,
    buildings: Query<(Entity, &Building, &GridTransform, Option<&PlayerRef>)>,
) {
    for (safety_entity, mut safety, safety_transform, safety_player_ref) in &mut building_safeties {
        for (building_entity, building, building_transform, building_player_ref) in &buildings {
            if safety_entity == building_entity {
                continue;
            }

            if safety_player_ref.is_none() || building_player_ref.is_none() {
                continue;
            }

            let own_building = safety_player_ref == building_player_ref;

            let dist = (safety_transform.center_in_world() - building_transform.center_in_world())
                .length();
            let arrival_time = dist / building.building_type().ants_produced().base_stats().speed;
            let arrival_time_sqr = arrival_time * arrival_time;

            if !own_building {
                safety.spatial -= 1.0 / arrival_time_sqr;
            } else {
                safety.spatial += 1.0 / arrival_time_sqr;
            }
        }
    }
}

fn update_incomming_safety(
    mut building_safeties: Query<(
        Entity,
        &mut BuildingSafety,
        &GridTransform,
        Option<&PlayerRef>,
    )>,
    ants: Query<(&AntTarget, &AntStats, &Transform, &PlayerRef)>,
) {
    for (entity, mut safety, grid_transform, safety_player_ref) in &mut building_safeties {
        for (ant_target, ant_stats, ant_transform, ant_player_ref) in &ants {
            if ant_target.0 != entity {
                continue;
            }

            let dist = (grid_transform.center_in_world() - ant_transform.translation.xy()).length();
            let own = Some(ant_player_ref) == safety_player_ref;

            let speed = ant_stats.speed;

            let arrival_time = (dist / speed) - 3.0;
            let arrival_time = f32::max(arrival_time, 1.0);

            if own {
                safety.incomming += 1.0 / arrival_time;
            } else {
                let health_power = ant_stats.health.percent();
                let attack_danger = ant_stats.attack_power * health_power;
                safety.incomming -= attack_danger / arrival_time;
            }
        }
    }
}

fn _print_building_safety(
    mut gizmos: Gizmos,
    building_safeties: Query<(&BuildingSafety, &GridTransform), With<BuildingSelected>>,
) {
    for (safety, grid_transform) in &building_safeties {
        let text_pos = grid_transform.center_in_world();
        let text_pos = text_pos - (grid_transform.size_world().y / 2.0) * Vec2::Y - Vec2::Y * 30.0;

        let total_safety = safety.spatial + safety.incomming + safety.defense;

        let text = format!(
            "Safety: {:.2}\nSpatial: {:.2}\nIncomming: {:.2}\nDefense: {:.2}\nDamage: {}",
            total_safety,
            safety.spatial,
            safety.incomming,
            safety.defense,
            safety.incomming_damage()
        );

        gizmos.text_2d(text_pos, &text, 7.0, Vec2::ZERO, Color::WHITE);
    }
}
