use bevy::ecs::entity::Entity;

use crate::{game::bot::bot_view::BotView, world_grid::CELL_SIZE};

#[derive(Clone)]
pub struct Brain {}

impl Default for Brain {
    fn default() -> Self {
        Self::simple()
    }
}

impl Brain {
    pub fn simple() -> Self {
        Self {}
    }

    pub fn take_action(&self, bot_view: &BotView) -> BotAction {
        let mut bot_action = BotAction::None;
        let mut action_score = 0.0;

        for bot_building in &bot_view.bot_buildings {
            for enemy_building in &bot_view.enemy_buildings {
                let action = BotAction::MoveOrder {
                    source_building: bot_building.entity,
                    target_building: enemy_building.entity,
                };

                let mut score = 0.0;
                let dist = (bot_building.world_pos - enemy_building.world_pos).length();
                score += self.dist_score(dist);
                score += self.pop_score(bot_building.inhabitants, bot_building.max_inhabitants);
                score += self.pop_diff_score(bot_building.inhabitants, enemy_building.inhabitants);

                if score > action_score {
                    bot_action = action;
                    action_score = score;
                }
            }
        }

        // println!("{:?} -> {}", bot_action, action_score);

        bot_action
    }

    fn dist_score(&self, dist: f32) -> f32 {
        if dist == 0.0 {
            return 1000000.0;
        }

        ((CELL_SIZE * 20.0) / (dist * dist)) - (dist / 4.0)
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
}

#[derive(Debug)]
pub enum BotAction {
    None,
    MoveOrder {
        source_building: Entity,
        target_building: Entity,
    },
}
