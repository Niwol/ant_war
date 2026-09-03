use bevy::{
    asset::RenderAssetUsages,
    camera::{ImageRenderTarget, RenderTarget, Viewport},
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};

use crate::{
    game::{
        building::{self, building_type::BuildingType},
        player::PlayerColor,
    },
    map::Map,
    menu::{
        MenuState,
        game_preparation_menu::{
            MAP_PREVIEW_IMAGE_SIZE,
            menu_backend::{MapLoadingState, SelectedMap},
        },
    },
};

pub const PREVIEW_SCENE_OFFSET: Vec3 = Vec3 {
    x: 5000.0,
    y: 5000.0,
    z: 0.0,
};

pub fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(MenuState::GamePreparation),
        create_map_preview_image,
    );
    app.add_systems(OnEnter(MapLoadingState::Loaded), spawn_preview_scene);

    app.add_systems(
        OnExit(MenuState::GamePreparation),
        remove_map_preview_elements,
    );
}

#[derive(Component, Default, Clone)]
struct PreviewSceneElement;

#[derive(Component, Default, Clone)]
struct MapPreviewBuilding;

#[derive(Component, Default, Clone)]
pub struct MainBuildingIndex(pub usize);

fn spawn_preview_scene(
    mut commands: Commands,
    selected_map: Res<SelectedMap>,
    maps: Res<Assets<Map>>,
    mut preview_scene_camera: Single<&mut Projection, With<PreviewSceneCamera>>,
    map_preview_buildings: Query<Entity, With<MapPreviewBuilding>>,
) {
    for entity in map_preview_buildings {
        commands.entity(entity).despawn();
    }

    let map = maps.get(&selected_map.handle).unwrap();

    for building_info in map.building_infos_as_vec() {
        let mut grid_transform = building_info.grid_transform;
        let center = grid_transform.center_in_world();

        grid_transform
            .update_from_world(center + PREVIEW_SCENE_OFFSET.xy() - map.size_world() / 2.0);

        let mut building = commands.spawn_scene(bsn! {
            PreviewSceneElement
            MapPreviewBuilding
            Sprite {
                image: building::asset_paths::get_path(building_info.building_type, PlayerColor::Neutral),
            }
            template_value(grid_transform)
        });

        match building_info.building_type {
            BuildingType::House => (),
            BuildingType::MainBuilding { index } => {
                building.insert(MainBuildingIndex(index));
            }
            BuildingType::Tower => (),
        }
    }

    let preview_scene_camera = preview_scene_camera.as_mut();

    if let Projection::Orthographic(orthographic) = preview_scene_camera {
        let max_element = map.size_world().max_element();
        let aspect = max_element / MAP_PREVIEW_IMAGE_SIZE.max_element() as f32;
        orthographic.scale = aspect;
    }
}

#[derive(Resource)]
pub struct MapPreviewImage {
    pub handle: Handle<Image>,
}

#[derive(Component, Default, Clone)]
struct PreviewSceneCamera;

fn create_map_preview_image(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut image = Image::new_fill(
        Extent3d {
            width: MAP_PREVIEW_IMAGE_SIZE.x,
            height: MAP_PREVIEW_IMAGE_SIZE.y,
            ..Default::default()
        },
        TextureDimension::D2,
        &[0, 0, 0, 0],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    );
    image.texture_descriptor.usage =
        TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_DST;

    let handle = images.add(image);
    let mut camera = commands.spawn_scene(bsn! {
        Camera2d
        PreviewSceneCamera
        Transform::from_translation(PREVIEW_SCENE_OFFSET)
    });

    camera.insert((
        RenderTarget::Image(ImageRenderTarget {
            handle: handle.clone(),
            scale_factor: 1.0,
        }),
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(1.0, 1.0, 1.0)),
            order: 3,
            viewport: Some(Viewport {
                physical_size: MAP_PREVIEW_IMAGE_SIZE,
                ..Default::default()
            }),
            ..Default::default()
        },
    ));

    commands.insert_resource(MapPreviewImage { handle });
}

fn remove_map_preview_elements(
    mut commands: Commands,
    map_preview_camera: Single<Entity, With<PreviewSceneCamera>>,
    preview_elements: Query<Entity, With<PreviewSceneElement>>,
) {
    commands.entity(*map_preview_camera).despawn();

    for entity in &preview_elements {
        commands.entity(entity).despawn();
    }

    commands.remove_resource::<MapPreviewImage>();
}
