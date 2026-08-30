use bevy::prelude::*;

use crate::{
    game::{
        ant::{Ant, ant_spawner::AntSpawner},
        building::{BuildingId, inhabitants::Inhabitants},
        game_info::{GameState, StartGameInfo, end_game::EndGameInfo},
        player::PlayerRef,
    },
    world_grid::WorldGrid,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::StartingDecount),
        (add_starting_decount, init_game),
    );
    app.add_systems(OnExit(GameState::StartingDecount), remove_starting_decount);
    app.add_systems(
        Update,
        update_starting_decount.run_if(in_state(GameState::StartingDecount)),
    );
}

#[derive(Resource)]
pub struct StartingDecount {
    seconds_left: f32,
}

#[derive(Component, Default, Clone)]
struct StartingDecountText;

fn add_starting_decount(mut commands: Commands, world_grid: Res<WorldGrid>) {
    let starting_decount = StartingDecount { seconds_left: 3.0 };

    let world_center = world_grid.center();

    commands.spawn_scene(bsn! {
        StartingDecountText
        Text2d::new(format!("{}", starting_decount.seconds_left))
        TextFont { font_size: px(50.0) }
        TextColor(Color::srgb(0.0, 1.0, 0.0))
        Transform::from_xyz(world_center.x, world_center.y, 0.0)
    });

    commands.insert_resource(starting_decount);
}

fn remove_starting_decount(
    mut commands: Commands,
    decount_text: Single<Entity, With<StartingDecountText>>,
) {
    commands.entity(*decount_text).despawn();

    commands.remove_resource::<StartingDecount>();
}

fn update_starting_decount(
    time: Res<Time>,
    mut starting_decount: ResMut<StartingDecount>,
    mut decount_text: Single<&mut Text2d, With<StartingDecountText>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    starting_decount.seconds_left -= time.delta_secs();

    if starting_decount.seconds_left > 0.0 {
        decount_text.0 = format!("{}", starting_decount.seconds_left.ceil() as i32);
    } else if starting_decount.seconds_left > -1.0 {
        decount_text.0 = String::from("Start");
    } else {
        next_state.set(GameState::Playing { paused: false });
    }
}

fn init_game(
    mut commands: Commands,
    start_game_info: Res<StartGameInfo>,
    ants: Query<Entity, With<Ant>>,
    ant_spawners: Query<Entity, With<AntSpawner>>,
    buildings: Query<(Entity, &BuildingId)>,
) {
    for entity in ants {
        commands.entity(entity).despawn();
    }

    for entity in ant_spawners {
        commands.entity(entity).despawn();
    }

    for (building_entity, building_id) in &buildings {
        if let Some(player_ref) = start_game_info.building_assignements.get(building_id) {
            commands.entity(building_entity).insert(*player_ref);
        } else {
            commands.entity(building_entity).remove::<PlayerRef>();
        }

        commands.entity(building_entity).insert(Inhabitants::new(5));
    }

    commands.insert_resource(EndGameInfo { winner: None });
}
