use crate::game::components::*;
use crate::game::resources::*;
use crate::game::systems::GameState;

pub struct EventProcessor;
impl EventProcessor
{
    pub fn apply_events(game_state: &mut GameState, events: &Vec<GameEvent>) // -> Result?
    {
        events.iter()
              .for_each(|ev|
                  match ev
                  {
                      GameEvent::Move { id, orig, dst } =>
                      {
                          let obj = game_state.objects.get_mut(&id).unwrap();
                          if let Some(Component::Movable { pos, max_speed: _ }) = obj.components.get_mut(&ComponentID::MovableID)
                          {
                              assert!(*pos == *orig, "Mismatch on Movable.pos and Move.orig for object {:?}", id);
                              *pos = *dst;
                          }
                          game_state.map.move_obj(orig, dst);
                      },
                      _ => {}
                  });
    }
}



#[cfg(test)]
mod test
{
    use super::*;
    use super::super::ObjectInitializer;
    use std::collections::{HashMap};

    #[test]
    fn apply_move()
    {
        let mut map = Map::<ID>::new_with_dimensions((10, 10));
        let mut objs = HashMap::<ID, Object>::new();

        let components = vec![ Component::Movable { pos: (0, 0), max_speed: 1 } ];
        let obj = Object::new_with_components(ID::new_with_value(1), Owner::Player1, components);
        objs.insert(obj.id, obj);

        let mut game_state = GameState { map: &mut map, objects: &mut objs };

        ObjectInitializer::initialize(&mut game_state);

        let evs = vec![GameEvent::Move {id: ID::new_with_value(1), orig: (0,0), dst: (1,1)}];

        EventProcessor::apply_events(&mut game_state, &evs);

        let o = game_state.objects.get(&ID::new_with_value(1)).unwrap();
        match o.components.get(&ComponentID::MovableID)
        {
            Some(Component::Movable { pos, max_speed: _ }) =>
            {
                assert!(*pos == (1,1), "MoveEvent didn't move object");
            },
            _ => 
            {
                assert!(false, "Something went really wrong");
            }
        }
    }
}
