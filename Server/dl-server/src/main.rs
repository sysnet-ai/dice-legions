use std::collections::{HashMap};

mod ecs_lite;

use ecs_lite::components::*;
use ecs_lite::resources::*;
use ecs_lite::systems::*;

struct CombatHandler;
impl CombatHandler
{
    fn handle_combat(&self, attacker: &Object, target: &Object, world: &GameController) -> Vec<GameEvent>
    {
        /*
        if let Component::Attackable { max_health } = attacker.components.get(&ComponentID::AttackableID).unwrap() // Let it panic
        {
            if let Component::Attackable { max_health } = target.components.get(&ComponentID::AttackableID).unwrap()
            {
                //
                 
            }
        }
        */

        vec![]
    }
}

struct GameController
{
    map: Map<ID>,
    objects: HashMap<ID, Object>,
}

impl GameController
{
    pub fn new(map: Map<ID>, objs: HashMap<ID, Object>) -> GameController
    {
        let mut gc = GameController
        {
            map: map,
            objects: objs,
        };

        gc
    }

    pub fn start(&mut self /*, ..*/)
    {
        ObjectInitializer::initialize(&mut self.game_state());

        let mut cur_player = Owner::Player1;
        //while(!stop_criteria)
        //{
        
            let actions = ActionGatherer::available_actions(&self.game_state(), &cur_player);

            // Send actions to player, wait for response

            cur_player = cur_player.other();
        //}
        
    }

    fn game_state(&mut self) -> GameState
    {
        GameState { map: &mut self.map, objects: &mut self.objects }
    }
}

#[allow(dead_code)]
fn main() 
{
}

#[cfg(test_obs)]
mod test
{
    use super::*;
    use std::collections::{HashSet};
 
    
}
