use std::convert::TryInto;
use std::collections::{HashMap, HashSet};


use crate::ecs_lite::components::*;
use crate::ecs_lite::resources::*;

pub struct GameState<'a>
{
    pub map: &'a mut Map<ID>,
    pub objects: &'a mut HashMap<ID, Object>
}

mod action_processor;
mod action_gatherer;
mod action_validator;
mod event_processor;
mod initializer;

pub use self::action_processor::*;
pub use self::action_gatherer::*;
pub use self::action_validator::*;
pub use self::event_processor::*;
pub use self::initializer::*;








