use std::collections::{HashMap};

mod game;

use game::components::*;
use game::resources::*;
use game::systems::*;

struct GameController
{
    map: Map<ID>,
    objects: HashMap<ID, Object>,
}

impl GameController
{
    pub fn new(map: Map<ID>, objs: HashMap<ID, Object>) -> GameController
    {
        GameController
        {
            map: map,
            objects: objs,
        }
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
