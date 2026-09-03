use bevy::prelude::*;

use crate::{
    AppState,
    game::{
        ant::Ant,
        building::{
            Building, BuildingProps, building_type::BuildingType, inhabitants::Inhabitants,
        },
        player::{PlayerColor, PlayerRef},
        projectiles::Projectile,
    },
    world_grid::grid_transform::GridTransform,
};

pub const TOWER_RANGE: f32 = 100.0;
const TOWER_SHOOT_TIME: f32 = 1.0;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, draw_tower_range.run_if(in_state(AppState::InGame)));
    app.add_systems(
        Update,
        update_tower_timer.run_if(in_state(AppState::InGame)),
    );

    app.add_systems(Update, shoot_at_ants);
}

#[derive(SceneComponent, Clone)]
#[scene(TowerProps)]
pub struct Tower {
    shoot_timer: Timer,
}

impl Default for Tower {
    fn default() -> Self {
        Self {
            shoot_timer: Timer::from_seconds(TOWER_SHOOT_TIME, TimerMode::Once),
        }
    }
}

#[derive(Default)]
pub struct TowerProps {
    pub building_props: BuildingProps,
}

impl Tower {
    fn scene(props: TowerProps) -> impl Scene {
        let building_props = props.building_props;
        let image_path = super::asset_paths::get_path(BuildingType::Tower, PlayerColor::Neutral);

        bsn! {
            @Building {
                building_type: BuildingType::Tower,
                @building_id: {building_props.building_id},
                @grid_transform: {building_props.grid_transform}
            }

            Inhabitants::new(5, 30, None)

            Sprite {
                image: image_path
            }
        }
    }

    pub fn can_shoot(&self) -> bool {
        self.shoot_timer.is_finished()
    }
}

fn draw_tower_range(mut gizmos: Gizmos, towers: Query<&GridTransform, With<Tower>>) {
    for grid_transform in &towers {
        let center = grid_transform.center_in_world();

        gizmos.circle_2d(center, TOWER_RANGE, Color::WHITE);
    }
}

fn update_tower_timer(time: Res<Time>, mut towers: Query<&mut Tower>) {
    for mut tower in &mut towers {
        tower.shoot_timer.tick(time.delta());
    }
}

fn shoot_at_ants(
    mut commands: Commands,
    mut towers: Query<(&mut Tower, &GridTransform, Option<&PlayerRef>)>,
    ants: Query<(Entity, &Transform, &PlayerRef), With<Ant>>,
) {
    for (mut tower, grid_transform, tower_player_ref) in &mut towers {
        if !tower.can_shoot() {
            continue;
        }

        let mut closest_ant = None;
        let tower_center = grid_transform.center_in_world();

        for (ant_entity, transform, ant_player_ref) in &ants {
            if let Some(player_ref) = tower_player_ref
                && player_ref == ant_player_ref
            {
                continue;
            }

            let dist = (tower_center - transform.translation.xy()).length();
            match closest_ant {
                Some((entity, closest_dist)) => {
                    if dist < closest_dist {
                        closest_ant = Some((entity, dist));
                    }
                }
                None => {
                    if dist < TOWER_RANGE {
                        closest_ant = Some((ant_entity, dist));
                    }
                }
            }
        }

        if let Some((ant_entity, _)) = closest_ant {
            commands.spawn_scene(bsn! {
                @Projectile {
                    target: ant_entity,
                    @spawn_position: tower_center
                }
            });

            tower.shoot_timer.reset();
        }
    }
}
