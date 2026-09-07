use bevy::prelude::*;

use crate::game::{building::Building, game_info::GameState, player::PlayerRef};

pub struct InhabitantsPlugin;
impl Plugin for InhabitantsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (update_inhabitants_text_position, update_inhabitants_text),
        );
        app.add_systems(
            Update,
            grow_inhabitants.run_if(in_state(GameState::Playing { paused: false })),
        );

        app.add_systems(
            Update,
            shrink_inhabitants.run_if(in_state(GameState::Playing { paused: false })),
        );

        app.add_observer(on_add_inhabitants);
    }
}

#[derive(Component, Default, Clone)]
#[require(ShrinkTimer)]
pub struct Inhabitants {
    current: i32,
    max: i32,
    grow_timer: Option<Timer>,
}

#[derive(Component, Clone)]
struct ShrinkTimer(Timer);

impl Default for ShrinkTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

impl Inhabitants {
    pub fn new(inhabitants: i32, max_inhabitants: i32, grow_timer: Option<Timer>) -> Self {
        Self {
            current: inhabitants,
            max: max_inhabitants,
            grow_timer,
        }
    }

    pub fn current(&self) -> i32 {
        self.current
    }

    pub fn max_inhabitants(&self) -> i32 {
        self.max
    }

    pub fn set(&mut self, current: i32) {
        self.current = current
    }

    pub fn take(&mut self, amount: i32) {
        self.current = i32::max(0, self.current - amount);
    }

    pub fn add(&mut self, amount: i32) {
        self.current += amount;
    }
}

#[derive(Component, Default, Clone)]
struct InhabitantsText;

fn on_add_inhabitants(
    add: On<Add, Inhabitants>,
    mut commands: Commands,
    inhabitants: Query<&Inhabitants>,
    images: Res<Assets<Image>>,
    sprites: Query<&Sprite>,
) {
    let inhabitants = inhabitants.get(add.entity).unwrap();
    let currnet_inhabitants = inhabitants.current;

    let mut sprite_size = Vec2::ONE;

    if let Ok(sprite) = sprites.get(add.entity) {
        let image = images.get(&sprite.image);

        sprite_size = if let Some(image) = image {
            image.size_f32()
        } else if let Some(custom_size) = sprite.custom_size {
            custom_size
        } else {
            Vec2::ONE
        };
    }

    let translation = Vec3::new(0.0, sprite_size.y / 2.0 + 10.0, 0.0);

    let text = commands
        .spawn_scene(bsn! {
            InhabitantsText
            Text2d::new(format!("{}", currnet_inhabitants))
            TextFont {
                font_size: FontSize::Px(20.0)
            }

            Transform::from_translation(translation)
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
    inhabitants: Query<(&Inhabitants, &Children), Changed<Inhabitants>>,
) {
    for (inhabitants, children) in &inhabitants {
        for child in children {
            if let Ok(mut text) = texts.get_mut(*child) {
                *text = Text2d::new(format!("{}", inhabitants.current));
            }
        }
    }
}

fn grow_inhabitants(time: Res<Time>, mut inhabitants: Query<&mut Inhabitants, With<PlayerRef>>) {
    for mut inhabitants in &mut inhabitants {
        if inhabitants.current >= inhabitants.max {
            continue;
        }

        if let Some(grow_timer) = &mut inhabitants.grow_timer {
            grow_timer.tick(time.delta());

            if grow_timer.just_finished() {
                inhabitants.current += 1;
            }
        }
    }
}

fn shrink_inhabitants(
    time: Res<Time>,
    mut inhabitants: Query<(&mut Inhabitants, &mut ShrinkTimer), With<PlayerRef>>,
) {
    for (mut inhabitants, mut shrink_timer) in &mut inhabitants {
        if inhabitants.current > inhabitants.max {
            shrink_timer.0.tick(time.delta());

            if shrink_timer.0.just_finished() {
                inhabitants.current -= 1;
            }
        } else {
            shrink_timer.0.reset();
        }
    }
}
