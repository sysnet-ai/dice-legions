use crate::game::components::*;
use crate::game::resources::*;
use crate::game::systems::GameState;

pub struct ActionValidator;
impl ActionValidator
{
    pub fn validate_action(game_state: &GameState, action:&GameAction) -> bool
    {
        true 
    }
}
