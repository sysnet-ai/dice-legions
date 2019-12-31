
use serde::{Serialize, Deserialize};

use crate::game::conf_random::*;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct ID(pub usize);

const IDNONE:ID = ID(0);
impl Default for ID
{
    fn default() -> Self 
    {
        IDNONE
    }
}
impl ID
{
    pub fn new_with_value(v:usize) -> ID
    {
        assert!(v != IDNONE.0, "Can't create an ID wit v = 0");
        ID(v)
    }
}

pub type GridPosition = (usize, usize);


//TODO: Where does this go?
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DiceType
{
    Trick,
    Smash,
    Chain
}

impl DiceType
{
    fn weak_vs(&self, other: &Self) -> bool
    {
        match self
        {
            DiceType::Trick =>
                match other
                {
                    DiceType::Smash => true,
                    _ => false
                },
            DiceType::Smash =>
                match other
                {
                    DiceType::Chain => true,
                    _ => false
                }
            DiceType::Chain =>
                match other
                {
                    DiceType::Trick => true,
                    _ => false
                }
        }
    }

    fn strong_vs(&self, other: &Self) -> bool
    {
        match self
        {
            DiceType::Trick =>
                match other
                {
                    DiceType::Chain => true,
                    _ => false
                },
            DiceType::Smash =>
                match other
                {
                    DiceType::Trick => true,
                    _ => false
                }
            DiceType::Chain =>
                match other
                {
                    DiceType::Smash => true,
                    _ => false
                }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct DiceRoll
{
    pub dice_type: DiceType,
    pub value: i8,
}

impl DiceRoll
{
    pub fn new_with_value(dt: DiceType, v: i8) -> DiceRoll
    {
        DiceRoll
        {
            dice_type: dt,
            value: v 
        }
    }

    pub fn  value_as_seen_by(&self, other: &Self) -> i8
    {
        if self.dice_type.strong_vs(&other.dice_type)
        {
            self.value + 1
        }
        else if self.dice_type.weak_vs(&other.dice_type)
        {
            self.value - 1
        }
        else 
        {
            self.value
        }
    }
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub enum Component
{
    Movable    { pos: GridPosition, max_speed: i32 },
    Attackable { max_health: i32, attacker_dice: Vec<DiceType>, defender_dice: Vec<DiceType> },
}

impl Component
{
    pub fn get_component_id(&self) -> ComponentID
    {
        match self 
        {
            Component::Movable { pos: _, max_speed: _ } =>
            {
                ComponentID::MovableID
            },
            Component::Attackable { max_health: _, attacker_dice: _, defender_dice: _ } =>
            {
                ComponentID::AttackableID
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ComponentID
{
    MovableID,
    AttackableID,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum StatID
{
    Health,
    Speed,
}


//TODO: Move these out
#[allow(dead_code)]
#[derive(PartialEq, Debug, Serialize, Deserialize)]
pub enum GameEvent
{
    Move      { id: ID, orig: GridPosition, dst: GridPosition },
    StatDelta { id: ID, by: i32, stat: StatID },
    Combat    { attacker_id: ID, target_id: ID },
    Hit       { attacker_dice: DiceRoll, defender_dice: DiceRoll },
    Death     { id: ID },

    EventContainer    { contained_events: Vec<GameEvent> },
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GameAction
{
    MoveTo   { id: ID, orig: GridPosition, dst: GridPosition },
    AttackOn { id: ID, target: ID, orig: GridPosition, dst: GridPosition },
}
