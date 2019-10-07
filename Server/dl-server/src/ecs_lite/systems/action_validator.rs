use crate::ecs_lite::components::*;
use crate::ecs_lite::resources::*;
use crate::ecs_lite::systems::GameState;

pub struct ActionValidator;
impl ActionValidator
{
    pub fn validate_actions(game_state: &GameState, actions:&Vec<GameAction>) // -> ????
    {
    }
}
