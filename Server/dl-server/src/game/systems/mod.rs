use serde::{Serialize, Serializer, Deserialize};
use serde::ser::SerializeStruct;

use std::convert::TryInto;
use std::collections::{HashMap, HashSet};

use crate::game::components::*;
use crate::game::resources::*;
use crate::game::conf_random::*;

// TODO: Move this out
//
//
#[derive(Serialize, Deserialize)]
pub enum GamePhase
{
    GameStart,
    GatherActions,
    ProcessActions
}

pub struct GameState
{
    pub map: Map<ID>,
    pub objects: HashMap<ID, Object>,
    pub rand_ctx: RandomCtx,
    pub phase: GamePhase,
    pub cur_player: Owner,
}

impl Serialize for GameState
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
    S: Serializer, 
    {
        let mut s = serializer.serialize_struct("GameState", 4)?;
        s.serialize_field("map", &self.map)?;
        s.serialize_field("objects", &self.objects)?;
        s.serialize_field("phase", &self.phase)?;
        s.serialize_field("cur_player", &self.cur_player)?;
        s.end() 
    }
}

impl GameState 
{
    pub fn new(map: Map<ID>, objs: HashMap<ID, Object>) -> GameState
    {
        GameState
        {
            map: map, 
            objects: objs,
            rand_ctx: RandomCtx::new_unseeded("New GameState".to_string()),
            phase: GamePhase::GameStart,
            cur_player: Owner::Player1,
        }
    }
}

mod action_processor;
mod action_gatherer;
mod action_validator;
mod combat_processor;
mod event_processor;
mod initializer;

pub use self::action_processor::*;
pub use self::action_gatherer::*;
pub use self::action_validator::*;
pub use self::combat_processor::*;
pub use self::event_processor::*;
pub use self::initializer::*;








