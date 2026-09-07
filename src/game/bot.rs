use bevy::prelude::*;

use crate::game::{
    bot::{
        bot_view::{BotView, BotViewPlugin},
        brain::Brain,
    },
    building::MoveOrder,
    game_info::GameState,
};

pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BotViewPlugin);

        app.add_systems(
            Update,
            bot_actions.run_if(in_state(GameState::Playing { paused: false })),
        );
    }
}

pub mod bot_view;
pub mod brain;

#[derive(Component, Default, Clone)]
#[require(BotView)]
pub struct Bot {
    brain: Brain,
}

impl Bot {
    pub fn new(brain: Brain) -> Self {
        Self { brain }
    }
}

fn bot_actions(mut commands: Commands, bots: Query<(&Bot, &BotView)>) {
    for (bot, bot_view) in &bots {
        let bot_action = bot.brain.take_action(bot_view);

        match bot_action {
            brain::BotAction::None => (),
            brain::BotAction::MoveOrder {
                source_building,
                target_building,
            } => {
                commands.trigger(MoveOrder {
                    building: source_building,
                    target: target_building,
                });
            }
        }
    }
}
