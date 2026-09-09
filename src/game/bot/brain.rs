use std::time::Duration;

use bevy::prelude::*;

use crate::game::{
    bot::bot_view::{BotView, BuildingView},
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

    pub fn take_action(&self, bot_view: &BotView) -> BotAction {
        let mut bot_action = BotAction::None;
        let mut action_score = 0.0;

        for bot_building in &bot_view.bot_buildings {
            for other_building in &bot_view.all_buildings {
                if bot_building.entity == other_building.entity {
                    continue;
                }

                let action = BotAction::MoveOrder {
                    source_building: bot_building.entity,
                    target_building: other_building.entity,
                };

                let mut score = 0.0;

                match other_building.own {
                    true => {
                        let reinforcement_need = self.building_reinforcement_need(&other_building);
                        score += self.reinforcement_score(&bot_building, reinforcement_need);
                    }
                    false => {
                        score +=
                            self.pop_score(bot_building.inhabitants, bot_building.max_inhabitants);
                        score += self.source_building_attack_score(&bot_building);
                        score += self
                            .pop_diff_score(bot_building.inhabitants, other_building.inhabitants);

                        score += self.building_type_score(other_building.building_type);
                    }
                }
                let dist = (bot_building.world_pos - other_building.world_pos).length();
                score += self.dist_score(dist);
                score += self.timer_score();

                if score > action_score {
                    bot_action = action;
                    action_score = score;
                }
            }
        }

        bot_action
    }

    fn building_reinforcement_need(&self, building_view: &BuildingView) -> f32 {
        let building_type_need = match building_view.building_type {
            BuildingType::House => 1.0,
            BuildingType::HeadQuarter { index: _ } => 0.0,
            BuildingType::Tower => 10.0,
            BuildingType::Casern => 5.0,
            BuildingType::Walls => 15.0,
        };

        let fill_need =
            building_view.max_inhabitants as f32 / (building_view.inhabitants as f32 + 30.0);

        building_type_need * fill_need
    }

    fn reinforcement_score(&self, source_building: &BuildingView, reinforcement_need: f32) -> f32 {
        let building_type_score = match source_building.building_type {
            BuildingType::House => 5.0,
            BuildingType::HeadQuarter { index: _ } => 10.0,
            BuildingType::Tower => -20.0,
            BuildingType::Casern => -10.0,
            BuildingType::Walls => -5.0,
        };

        let fill_score =
            (source_building.inhabitants as f32 / source_building.max_inhabitants as f32) * 10.0;

        (building_type_score + fill_score) * reinforcement_need
    }

    fn building_type_score(&self, building_type: BuildingType) -> f32 {
        match building_type {
            BuildingType::House => 10.0,
            BuildingType::HeadQuarter { index: _ } => 30.0,
            BuildingType::Tower => 30.0,
            BuildingType::Casern => 20.0,
            BuildingType::Walls => 30.0,
        }
    }

    fn source_building_attack_score(&self, source_building: &BuildingView) -> f32 {
        match source_building.building_type {
            BuildingType::House => 10.0,
            BuildingType::HeadQuarter { index: _ } => 5.0,
            BuildingType::Tower => -10.0,
            BuildingType::Casern => 20.0,
            BuildingType::Walls => -50.0,
        }
    }

    fn dist_score(&self, dist: f32) -> f32 {
        -0.002 * (dist * dist) + 50.0
    }

    fn pop_score(&self, pop: i32, max_pop: i32) -> f32 {
        let pop_diff = (max_pop - pop) as f32;
        let pop_diff = pop_diff.clamp(0.001, max_pop as f32);

        let fill_ratio = 10.0 / pop_diff;

        (pop as f32 - 5.0) * 10.0 * fill_ratio
    }

    fn pop_diff_score(&self, pop: i32, enemy_pop: i32) -> f32 {
        let pop_diff = (pop - (enemy_pop * 2)) as f32;

        if pop_diff < 0.0 {
            pop_diff * 50.0
        } else {
            pop_diff * 5.0
        }
    }

    fn timer_score(&self) -> f32 {
        let remaining = self.action_timer.remaining_secs();

        -remaining * 50.0
    }
}

#[derive(Debug)]
pub enum BotAction {
    None,
    MoveOrder {
        source_building: Entity,
        target_building: Entity,
    },
}
