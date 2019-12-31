use crate::game::components::*;
use crate::game::resources::*;
use crate::game::systems::GameState;
use crate::game::systems::CombatProcessor;

pub struct ActionProcessor;
impl ActionProcessor
{
    pub fn process_actions(game_state: &mut GameState, actions:&Vec<GameAction>) -> Vec<GameEvent>
    {
        actions
        .iter()
        .map(|act| Self::process_action(game_state, act))
        .flatten()
        .collect()
    }

    pub fn process_action(game_state: &mut GameState, action:&GameAction) -> Vec<GameEvent>
    {
        match action
        {
            &GameAction::MoveTo { id, orig, dst } =>
            {
                vec![GameEvent::Move { id, orig, dst }]
            },
            &GameAction::AttackOn { id, target, orig, dst } =>
            {
                let mut evts = vec![GameEvent::Move { id, orig, dst }];

                evts.append(&mut CombatProcessor::process_combat(game_state, &id, &target));
                
                evts
            }
        }
    }
}
