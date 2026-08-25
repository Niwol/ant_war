use bevy::prelude::*;

use crate::{
    AppState,
    game::{
        InGameEntity,
        bot::Bot,
        building::Building,
        game_info::GameState,
        team::{Player, PlayerRef},
    },
};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedBuilding { building: None });
        app.add_systems(OnEnter(AppState::InGame), spawn_deselection_sprite);

        app.add_systems(Update, handle_key_input.run_if(in_state(AppState::InGame)));
    }
}

#[derive(EntityEvent)]
pub struct MoveOrder {
    pub entity: Entity,
    pub target: Entity,
}

#[derive(Resource)]
pub struct SelectedBuilding {
    building: Option<Entity>,
}

#[derive(Component, Default, Clone, Copy)]
#[require(InGameEntity)]
struct DeselectionSprite;

fn spawn_deselection_sprite(mut commands: Commands) {
    commands
        .spawn_scene(bsn! {
            InGameEntity
            DeselectionSprite
            Sprite {
                color: Color::srgba(0.0, 0.5, 0.0, 0.0),
                custom_size: Vec2::new(3000.0, 2000.0)
            }
            Pickable
            Transform::from_xyz(0.0, 0.0, -1.0)
        })
        .observe(
            |click: On<Pointer<Click>>, mut selected_building: ResMut<SelectedBuilding>| {
                if click.button == PointerButton::Primary {
                    selected_building.building = None;
                }
            },
        );
}

pub fn on_select(click: On<Pointer<Click>>, mut selected_building: ResMut<SelectedBuilding>) {
    if click.button == PointerButton::Primary {
        selected_building.building = Some(click.entity);
    }
}

pub fn on_order(
    click: On<Pointer<Click>>,
    selected_building: Res<SelectedBuilding>,
    buildings: Query<Option<&PlayerRef>, With<Building>>,
    players: Query<&Player, Without<Bot>>,
    mut commands: Commands,
) {
    if click.button != PointerButton::Secondary {
        return;
    }

    if let Some(building) = selected_building.building {
        let building_ref = buildings.get(building).unwrap();
        let Some(building_ref) = building_ref else {
            return;
        };

        if players.contains(building_ref.0) {
            commands.trigger(MoveOrder {
                entity: selected_building.building.unwrap(),
                target: click.entity,
            });
        }
    }
}

fn handle_key_input(
    input: Res<ButtonInput<KeyCode>>,
    current_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        match **current_state {
            GameState::StartingDecount => (),
            GameState::Playing { paused } => {
                next_state.set(GameState::Playing { paused: !paused });
            }
            GameState::GameOver => (),
        }
    }
}
