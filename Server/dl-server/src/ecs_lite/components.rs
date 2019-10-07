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
    /*
    fn apply(&mut self, ev: &GameEvent)
    {
        match (self, ev)
        {
            (Component::Movable { pos, max_speed: _ }, GameEvent::Move { id, orig, dst }) =>
            {
                assert!(*pos == *orig, "Mismatch on Movable.pos and Move.orig for object {:?}", id);
                *pos = *dst;
            },
            _ => {}
        }
    } */

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

impl GameAction
{
    pub fn process(&self) -> Vec<GameEvent>
    {
        /*
        match self
        {
            &GameAction::MoveTo { id, orig, dst } =>
            {
                vec![GameEvent::Move { id, orig, dst }]
            },
            &GameAction::AttackOn { id, target, orig, dst } =>
            {
                let mut evts = vec![GameEvent::Move { id, orig, dst }];
                let obj_attacker = world.objects.get(&id).unwrap();
                let obj_target = world.objects.get(&target).unwrap();

                evts.append(&mut world.combat.handle_combat(&obj_attacker, &obj_target, world));
                
                evts
            }
        }
        */
        vec![]
    }
}
