use bevy::prelude::*;

use crate::game::{
    building::{Building, BuildingSprites, BuildingType},
    team::{Player, PlayerColor, PlayerRef},
};

pub fn plugin(app: &mut App) {
    app.add_observer(on_add_house);
}

#[derive(Component, Default, Clone)]
pub struct HouseMarker;

fn on_add_house(
    add: On<Add, HouseMarker>,
    mut buildings: Query<(&mut Building, &mut Sprite, Option<&PlayerRef>)>,
    players: Query<&Player>,
    building_sprites: Res<BuildingSprites>,
) {
    let (mut building, mut sprite, player_ref) = buildings.get_mut(add.entity).unwrap();
    let color = match player_ref {
        Some(player_ref) => {
            let player = players.get(player_ref.0).unwrap();
            player.player_color()
        }

        None => PlayerColor::Neutral,
    };

    let image_handle = building_sprites.get(BuildingType::House, color);
    sprite.image = image_handle;

    building.max_inhabitants = 20;
    building.growable = true;
    building.grow_timer = Timer::from_seconds(0.5, TimerMode::Repeating);
}
