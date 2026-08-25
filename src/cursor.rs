use bevy::prelude::*;

use crate::MainCamera;

pub struct CursorPlugin;
impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Cursor>();

        app.add_systems(PreUpdate, update_cursor);
    }
}

#[derive(Resource, Default)]
pub struct Cursor {
    world_pos: Option<Vec2>,
}

impl Cursor {
    pub fn world_pos(&self) -> Option<Vec2> {
        self.world_pos
    }
}

fn update_cursor(
    window: Single<&Window>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut cursor: ResMut<Cursor>,
) {
    cursor.world_pos = None;

    let Some(cursor_position) = window.cursor_position() else {
        return;
    };

    let window_size = window.size();

    let ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;
    let ndc = Vec3 {
        x: ndc.x,
        y: -ndc.y, // Window pixels are from top to bottom
        z: 0.0,
    };

    let (camera, camera_transform) = camera.into_inner();

    let world = camera.ndc_to_world(camera_transform, ndc);

    if let Some(world) = world {
        cursor.world_pos = Some(world.xy());
    };
}
