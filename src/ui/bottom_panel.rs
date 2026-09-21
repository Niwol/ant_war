use bevy::prelude::*;

use crate::{
    AppState,
    game::building::{
        Building, building_selection::SelectedBuildings, building_types::BuildingType,
        inhabitants::Inhabitants,
    },
};

pub const BOTTOM_PANEL_HEIGHT: f32 = 120.0;

pub struct BottomPanelPlugin;
impl Plugin for BottomPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), bottom_panel.spawn());
        app.add_systems(
            Update,
            update_bottom_panel.run_if(resource_exists_and_changed::<SelectedBuildings>),
        );

        app.add_systems(Update, update_inhabitants_text);
    }
}

#[derive(Component, Default, Clone)]
pub struct BottomPanel;

#[derive(Component, FromTemplate)]
struct InhabitantsText {
    source_building: Entity,
}

fn bottom_panel() -> impl Scene {
    bsn! {
        DespawnOnExit<AppState>(AppState::InGame)
        BottomPanel

        Node {
            align_self: AlignSelf::FlexEnd,

            width: percent(100.0),
            height: px(BOTTOM_PANEL_HEIGHT),

            column_gap: px(5.0),
            border: px(2.0),
            border_radius: px(2.0),
            padding: px(5.0),
        }

        BorderColor::all(Color::WHITE)
        BackgroundColor(Color::BLACK)
    }
}

fn update_bottom_panel(
    mut commands: Commands,
    selected_buildings: Res<SelectedBuildings>,
    bottom_panel: Single<Entity, With<BottomPanel>>,
    buildings: Query<(&Building, &Inhabitants, &Sprite)>,
) {
    commands.entity(*bottom_panel).despawn_children();

    if selected_buildings.buildings.len() == 1 {
        let building_entity = selected_buildings.buildings.iter().next().unwrap();
        let (building, inhabitants, sprite) = buildings.get(*building_entity).unwrap();

        let building_info = commands
            .spawn_scene(building_info(
                *building_entity,
                building.building_type(),
                inhabitants.current(),
                sprite.image.clone(),
            ))
            .id();
        commands.entity(*bottom_panel).add_child(building_info);
    }
}

fn building_info(
    entity: Entity,
    building_type: BuildingType,
    inhabitants: i32,
    image: Handle<Image>,
) -> impl Scene {
    let building_name = building_type.name();

    let inhabitants_text = format!(
        "Inhabitants: {} / {}",
        inhabitants,
        building_type.max_inhabitants()
    );

    let ant_name = building_type.ants_produced().ant_name();
    let ant_stats = building_type.ants_produced().base_stats();

    let ants_text = format!(
        "Produces {}:
    - Speed: {}px/sec
    - Attack: {:.1}
    - Health: {:.1}",
        ant_name, ant_stats.speed, ant_stats.attack_power, ant_stats.health.max
    );

    bsn! {
        Node {
            column_gap: px(5.0)
        }

        Children [
            (
                ImageNode {
                    image
                }
            ),

            (
                Node {
                    flex_direction: FlexDirection::Column,
                    row_gap: px(2.0),
                }

                Children [
                    Text::new(building_name)
                    TextFont {
                        font_size: px(14.0)
                    },

                    InhabitantsText {
                        source_building: entity
                    }
                    Text::new(inhabitants_text)
                    TextFont {
                        font_size: px(12.0)
                    },

                    Text::new(ants_text)
                    TextFont {
                        font_size: px(12.0)
                    },
                ]
            )
        ]
    }
}

fn update_inhabitants_text(
    inhabitants: Query<&Inhabitants, Changed<Inhabitants>>,
    mut inhabitants_text: Single<(&mut Text, &InhabitantsText)>,
) {
    if let Ok(inhabitants) = inhabitants.get(inhabitants_text.1.source_building) {
        inhabitants_text.0.0 = format!(
            "Inhabitants: {} / {}",
            inhabitants.current(),
            inhabitants.max_inhabitants()
        );
    }
}
