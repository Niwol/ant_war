use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    bot::bot_view::{BotView, BuildingOwner, BuildingView},
    building::building_types::BuildingType,
};

#[derive(Clone)]
pub struct Brain {
    action_timer: Timer,
}

impl Default for Brain {
    fn default() -> Self {
        Self::eazy()
    }
}

impl Brain {
    pub fn eazy() -> Self {
        Self {
            action_timer: Timer::from_seconds(10.0, TimerMode::Once),
        }
    }

    pub fn normal() -> Self {
        Self {
            action_timer: Timer::from_seconds(5.0, TimerMode::Once),
        }
    }

    pub fn hard() -> Self {
        Self {
            action_timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }

    pub fn tick_action_timer(&mut self, delta: Duration) {
        self.action_timer.tick(delta);
    }

    pub fn reset_action_timer(&mut self) {
        self.action_timer.reset();
    }

    pub fn take_action(&self, bot_view: &BotView) -> Scores {
        let mut best_scores = Scores::new(BotAction::None);

        for bot_building in &bot_view.bot_buildings {
            for other_building in &bot_view.all_buildings {
                if bot_building.entity == other_building.entity {
                    continue;
                }

                let action = BotAction::MoveOrder {
                    source_building: bot_building.entity,
                    target_building: other_building.entity,
                };

                let scores = self.compute_scores(bot_view, action);

                if scores.total_score() > best_scores.total_score() {
                    best_scores = scores;
                }
            }
        }

        best_scores
    }

    pub fn compute_scores(&self, bot_view: &BotView, bot_action: BotAction) -> Scores {
        let mut scores = Scores::new(bot_action);

        match bot_action {
            BotAction::None => return scores,
            BotAction::MoveOrder {
                source_building,
                target_building,
            } => {
                let source_building_view = bot_view
                    .all_buildings
                    .iter()
                    .find(|building_view| building_view.entity == source_building)
                    .unwrap();
                let target_building_view = bot_view
                    .all_buildings
                    .iter()
                    .find(|building_view| building_view.entity == target_building)
                    .unwrap();

                let dist =
                    (source_building_view.world_pos - target_building_view.world_pos).length();

                let ant_base_stats = source_building_view
                    .building_type
                    .ants_produced()
                    .base_stats();

                let ant_speed = ant_base_stats.speed;

                let arriving_time = dist / ant_speed;

                let ants_send =
                    source_building_view.inhabitants / 2 + source_building_view.inhabitants % 2;

                match target_building_view.owner {
                    BuildingOwner::Own => {
                        let target_safety = target_building_view.building_safety;
                        let incomming_damage = target_safety.incomming_damage();

                        scores.reinforcement_score = incomming_damage * ants_send as f32;
                    }

                    BuildingOwner::Enemy | BuildingOwner::Neutral => {
                        let attack_power = ant_base_stats.attack_power * ants_send as f32;
                        let building_defense =
                            target_building_view.building_type.base_stats().defense
                                * target_building_view.inhabitants as f32;
                        scores.attack_score = attack_power - building_defense;
                        scores.attack_score += 10.0;
                    }
                }

                scores.general_score -= 20.0;
                scores.general_score += self.building_send_need(&source_building_view);
                scores.general_score -= arriving_time;
                scores.general_score += self.timer_score();
            }
        }

        scores
    }

    fn building_send_need(&self, building_view: &BuildingView) -> f32 {
        let building_target_pop_percentage = match building_view.building_type {
            BuildingType::House => 0.5,
            BuildingType::HeadQuarter { .. } => 0.5,
            BuildingType::Tower => 1.0,
            BuildingType::Casern => 0.5,
            BuildingType::Walls => 1.0,
        };

        let pop_percentage = building_view
            .building_type
            .inhabitants_percentage(building_view.inhabitants);

        (pop_percentage - building_target_pop_percentage) * 300.0
            + building_view.building_safety.spatial * 50.0
    }

    fn timer_score(&self) -> f32 {
        let remaining = self.action_timer.remaining_secs();

        -remaining * 50.0
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub enum BotAction {
    #[default]
    None,
    MoveOrder {
        source_building: Entity,
        target_building: Entity,
    },
}

#[derive(Debug, Default)]
pub struct Scores {
    pub action: BotAction,
    pub reinforcement_score: f32,
    pub attack_score: f32,
    pub general_score: f32,
}

impl Scores {
    pub fn new(bot_action: BotAction) -> Self {
        Self {
            action: bot_action,
            ..Default::default()
        }
    }

    pub fn total_score(&self) -> f32 {
        self.reinforcement_score + self.attack_score + self.general_score
    }
}
