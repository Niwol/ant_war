use bevy::prelude::*;

pub struct InvalidLocationPlugin;
impl Plugin for InvalidLocationPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_add_invalid_location);
        app.add_observer(on_remove_invalid_location);
    }
}

#[derive(Component, Default, Clone)]
pub struct InvalidLocation;

fn on_add_invalid_location(add: On<Add, InvalidLocation>, mut sprites: Query<&mut Sprite>) {
    let mut sprite = sprites.get_mut(add.entity).unwrap();

    let mut srgb_color = Srgba::from(sprite.color);
    srgb_color.red = 1.0;
    srgb_color.green = 0.0;
    srgb_color.blue = 0.0;

    sprite.color = Color::from(srgb_color);
}

fn on_remove_invalid_location(add: On<Remove, InvalidLocation>, mut sprites: Query<&mut Sprite>) {
    let mut sprite = sprites.get_mut(add.entity).unwrap();

    let mut srgb_color = Srgba::from(sprite.color);
    srgb_color.red = 1.0;
    srgb_color.green = 1.0;
    srgb_color.blue = 1.0;

    sprite.color = Color::from(srgb_color);
}
