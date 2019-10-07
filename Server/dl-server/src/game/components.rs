#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

#[allow(dead_code)]
pub enum Component
{
    Movable    { pos: GridPosition, max_speed: i32 },
    Attackable { max_health: i32 },
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
            Component::Attackable { max_health: _ } =>
            {
                ComponentID::AttackableID
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ComponentID
{
    MovableID,
    AttackableID,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum StatID
{
    Health,
    Speed,
}


//TODO: Move these out
#[allow(dead_code)]
pub enum GameEvent
{
    Move      { id: ID, orig: GridPosition, dst: GridPosition },
    StatDelta { id: ID, by: i32, stat: StatID },
    Combat    { attacker_id: ID, target_id: ID, evts_attacker: Vec<GameEvent>, evts_target: Vec<GameEvent> },

    Death     { id: ID },
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub enum GameAction
{
    MoveTo   { id: ID, orig: GridPosition, dst: GridPosition },
    AttackOn { id: ID, target: ID, orig: GridPosition, dst: GridPosition },
}
