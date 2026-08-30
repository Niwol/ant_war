use bevy::prelude::*;

pub struct SelectionRectPlugin;
impl Plugin for SelectionRectPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(start_selection_rect);

        app.add_systems(Update, update_selection_rect_border);
    }
}

#[derive(Event)]
pub struct StartSelectionRect {
    pub start_position: Vec2,
}

#[derive(SceneComponent, Default, Clone)]
pub struct SelectionRect {
    pub start_position: Vec2,
    pub size: Vec2,
}

#[derive(Component, Default, Clone)]
enum SelectionRectBorder {
    #[default]
    AdjacentHorizontal,
    OppositeHorizontal,
    AdjacentVertical,
    OppositeVertical,
}

impl SelectionRect {
    fn scene() -> impl Scene {
        bsn! {
            Transform::default()
            Visibility::default()

            Children [
                Sprite {
                    color: Color::WHITE,
                    custom_size: {Some(Vec2::ONE)}
                }
                template_value(SelectionRectBorder::AdjacentHorizontal),

                Sprite {
                    color: Color::WHITE,
                    custom_size: {Some(Vec2::ONE)}
                }
                template_value(SelectionRectBorder::OppositeHorizontal),

                Sprite {
                    color: Color::WHITE,
                    custom_size: {Some(Vec2::ONE)}
                }
                template_value(SelectionRectBorder::AdjacentVertical),

                Sprite {
                    color: Color::WHITE,
                    custom_size: {Some(Vec2::ONE)}
                }
                template_value(SelectionRectBorder::OppositeVertical),
            ]
        }
    }
}

fn start_selection_rect(start: On<StartSelectionRect>, mut commands: Commands) {
    commands.spawn_scene(bsn! {@SelectionRect {
        start_position: {start.start_position}
        size: Vec2::ZERO,
    }});
}

fn update_selection_rect_border(
    selection_rect: Single<&SelectionRect, Changed<SelectionRect>>,
    mut selection_rect_borders: Query<(&mut Transform, &SelectionRectBorder)>,
) {
    println!("Update Selection Rect Border");

    for (mut transform, border) in &mut selection_rect_borders {
        let start_pos = selection_rect.start_position;
        let size = selection_rect.size;

        let translation = match border {
            SelectionRectBorder::AdjacentHorizontal => Vec3 {
                x: start_pos.x + size.x / 2.0,
                y: start_pos.y,
                z: 0.0,
            },
            SelectionRectBorder::OppositeHorizontal => Vec3 {
                x: start_pos.x + size.x / 2.0,
                y: start_pos.y + size.y,
                z: 0.0,
            },
            SelectionRectBorder::AdjacentVertical => Vec3 {
                x: start_pos.x,
                y: start_pos.y + size.y / 2.0,
                z: 0.0,
            },
            SelectionRectBorder::OppositeVertical => Vec3 {
                x: start_pos.x + size.x,
                y: start_pos.y + size.y / 2.0,
                z: 0.0,
            },
        };

        let scale = match border {
            SelectionRectBorder::AdjacentHorizontal | SelectionRectBorder::OppositeHorizontal => {
                Vec3 {
                    x: size.x,
                    y: 1.0,
                    z: 1.0,
                }
            }
            SelectionRectBorder::AdjacentVertical | SelectionRectBorder::OppositeVertical => Vec3 {
                x: 1.0,
                y: size.y,
                z: 1.0,
            },
        };

        transform.translation = translation;
        transform.scale = scale;
    }
}
