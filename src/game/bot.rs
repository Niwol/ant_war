use bevy::prelude::*;

use crate::game::{
    bot::{
        bot_view::{BotView, BotViewPlugin},
        brain::{BotAction, Brain},
        building_safety::BuildingSafetyPlugin,
    },
    building::MoveOrder,
    game_info::GameState,
};

pub mod bot_view;
pub mod brain;
pub mod building_safety;

pub struct BotPlugin;
impl Plugin for BotPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BuildingSafetyPlugin, BotViewPlugin));

        app.add_systems(
            Update,
            (tick_brain_timers, bot_actions)
                .chain()
                .run_if(in_state(GameState::Playing { paused: false })),
        );
    }
}

#[derive(Component, Default, Clone)]
#[require(BotView)]
pub struct Bot {
    brain: Brain,
}

impl Bot {
    pub fn _new(brain: Brain) -> Self {
        Self { brain }
    }

    pub fn from_difficulty(bot_difficulty: BotDifficulty) -> Self {
        let brain = match bot_difficulty {
            BotDifficulty::Eazy => Brain::eazy(),
            BotDifficulty::Normal => Brain::normal(),
            BotDifficulty::Hard => Brain::hard(),
        };

        Self { brain }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum BotDifficulty {
    Eazy,
    Normal,
    Hard,
}

impl ToString for BotDifficulty {
    fn to_string(&self) -> String {
        match self {
            BotDifficulty::Eazy => "Eazy",
            BotDifficulty::Normal => "Normal",
            BotDifficulty::Hard => "Hard",
        }
        .to_string()
    }
}

fn tick_brain_timers(time: Res<Time>, mut bots: Query<&mut Bot>) {
    for mut bot in &mut bots {
        bot.brain.tick_action_timer(time.delta());
    }
}

fn bot_actions(mut commands: Commands, mut bots: Query<(&mut Bot, &BotView)>) {
    for (mut bot, bot_view) in &mut bots {
        let scores = bot.brain.take_action(bot_view);

        match scores.action {
            BotAction::None => (),
            _ => bot.brain.reset_action_timer(),
        }

        match scores.action {
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
