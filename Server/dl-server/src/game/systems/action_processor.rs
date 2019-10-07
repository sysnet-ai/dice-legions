use crate::game::components::*;
use crate::game::resources::*;
use crate::game::systems::GameState;

pub struct ActionProcessor;
impl ActionProcessor
{
    pub fn process_actions(game_state: &GameState, actions:&Vec<GameAction>) -> Vec<GameEvent>
    {
        actions
        .iter()
        .map(|act|
            match act
            {
                &GameAction::MoveTo { id, orig, dst } =>
                {
                    vec![GameEvent::Move { id, orig, dst }]
                },
                &GameAction::AttackOn { id, target, orig, dst } =>
                {
                    let evts = vec![GameEvent::Move { id, orig, dst }];
                    let obj_attacker = game_state.objects.get(&id).unwrap();
                    let obj_target = game_state.objects.get(&target).unwrap();

                    //evts.append(&mut game_state.combat.handle_combat(&obj_attacker, &obj_target, game_state));
                    
                    evts
                }
            })
        .flatten()
        .collect()
    }
}
