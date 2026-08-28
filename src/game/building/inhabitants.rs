use bevy::prelude::*;

use crate::game::building::Building;

pub fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (update_inhabitants_text_position, update_inhabitants_text),
    );

    app.add_observer(on_add_inhabitants);
}

#[derive(Component, Default, Clone, Copy)]
pub struct Inhabitants {
    total: i32,
    to_print: i32,
}

impl Inhabitants {
    pub fn new(inhabitants: i32) -> Self {
        Self {
            total: inhabitants,
            to_print: inhabitants,
        }
    }

    pub fn total(&self) -> i32 {
        self.total
    }

    pub fn take(&mut self, amount: i32) {
        self.total = i32::max(0, self.total - amount);
    }

    pub fn add(&mut self, amount: i32) {
        self.total += amount;
    }
}

#[derive(Component, Default, Clone)]
struct InhabitantsText;

fn on_add_inhabitants(
    add: On<Add, Inhabitants>,
    mut commands: Commands,
    inhabitants: Query<&Inhabitants>,
) {
    let inhabitants = *inhabitants.get(add.entity).unwrap();

    let text = commands
        .spawn_scene(bsn! {
            InhabitantsText
            Text2d::new(format!("{}", inhabitants.total))
            TextFont {
                font_size: FontSize::Px(20.0)
            }
        })
        .id();

    commands.entity(add.entity).add_child(text);
}

fn update_inhabitants_text_position(
    buildings: Query<(&Sprite, &Children), (Changed<Sprite>, With<Building>)>,
    mut texts: Query<(&mut Transform, &mut TextColor), With<InhabitantsText>>,
    images: Res<Assets<Image>>,
) {
    for (building_sprite, children) in &buildings {
        for child in children {
            if let Ok((mut transform, mut text_color)) = texts.get_mut(*child) {
                let image = images.get(&building_sprite.image);

                let sprite_size = if let Some(image) = image {
                    image.size_f32()
                } else if let Some(size) = building_sprite.custom_size {
                    size
                } else {
                    Vec2::ZERO
                };

                transform.translation = Vec3::new(0.0, sprite_size.y / 2.0 + 10.0, 0.0);
                text_color.0 = building_sprite.color;
            }
        }
    }
}

fn update_inhabitants_text(
    mut texts: Query<&mut Text2d>,
    mut inhabitants: Query<(&mut Inhabitants, &Children)>,
) {
    for (mut inhabitants, children) in &mut inhabitants {
        if inhabitants.total < inhabitants.to_print {
            inhabitants.to_print -= 1;
        } else if inhabitants.total > inhabitants.to_print {
            inhabitants.to_print += 1;
        }

        for child in children {
            if let Ok(mut text) = texts.get_mut(*child) {
                *text = Text2d::new(format!("{}", inhabitants.to_print));
            }
        }
    }
}
