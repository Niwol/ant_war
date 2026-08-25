use bevy::prelude::*;

use crate::{game::game_info::GameState, world_grid::WorldGrid};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::StartingDecount), add_starting_decount);
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
