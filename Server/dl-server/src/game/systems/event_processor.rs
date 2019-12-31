use crate::game::components::*;
use crate::game::resources::*;
use crate::game::systems::GameState;

pub struct EventProcessor;
impl EventProcessor
{
    pub fn apply_events(game_state: &mut GameState, mut events: Vec<GameEvent>) -> Vec<GameEvent>
    {
        let mut processed_events = vec![];

        while !events.is_empty()
        {
            let ev = events.pop().unwrap();
            events.append(&mut Self::apply_event(game_state, &ev));
            processed_events.push(ev);
        }

        processed_events
    }

    fn apply_event(game_state: &mut GameState, event: &GameEvent) -> Vec<GameEvent>
    {
        let mut generated_events = vec![];
        match event
        {
            &GameEvent::Move { id, orig, dst } => {
                let obj = game_state.objects.get_mut(&id).unwrap();
                if let Some(Component::Movable { pos, max_speed: _ }) = obj.components.get_mut(&ComponentID::MovableID)
                {
                    assert!(*pos == orig, "Mismatch on Movable.pos and Move.orig for object {:?}", id);
                    *pos = dst;
                }
                game_state.map.move_obj(&orig, &dst);
            },
            &GameEvent::StatDelta { id, by, stat } => {
                let obj = game_state.objects.get_mut(&id).unwrap();
                let v = obj.get_stat_value(&StatID::Health);
                obj.stats.insert(stat, v + by);

                if obj.get_stat_value(&StatID::Health) <= 0
                {
                    generated_events.push(GameEvent::Death { id: id });
                }
            },
            &GameEvent::Death { id } => {
                let obj = game_state.objects.get_mut(&id).unwrap();
                if let Some(Component::Movable { pos, max_speed: _ }) = obj.components.get(&ComponentID::MovableID)
                {
                    game_state.map.remove_obj(pos);
                }
            },
            GameEvent::EventContainer { contained_events } => {
                contained_events.iter().for_each(|ev| generated_events.append(&mut Self::apply_event(game_state, ev)));
            },
            _ => {
            }
        }

        generated_events
    }
}



#[cfg(test)]
mod test
{
    use super::*;

    use crate::game::conf_random::*;
    use crate::game::systems::ObjectInitializer;
    use std::collections::{HashMap};

    #[test]
    fn apply_move()
    {
        let map = Map::<ID>::new_with_dimensions((10, 10));
        let mut objs = HashMap::<ID, Object>::new();

        let components = vec![ Component::Movable { pos: (0, 0), max_speed: 1 } ];
        let obj = Object::new_with_components(ID::new_with_value(1), Owner::Player1, components);
        objs.insert(obj.id, obj);

        let mut game_state = GameState::new(map, objs);

        ObjectInitializer::initialize(&mut game_state);

        let evs = vec![GameEvent::Move {id: ID::new_with_value(1), orig: (0,0), dst: (1,1)}];

        let proc_evs = EventProcessor::apply_events(&mut game_state, evs);

        let o = game_state.objects.get(&ID::new_with_value(1)).unwrap();
        match o.components.get(&ComponentID::MovableID)
        {
            Some(Component::Movable { pos, max_speed: _ }) =>
            {
                assert!(*pos == (1,1), "MoveEvent didn't move object");
            },
            _ => 
            {
                panic!("Something went really wrong");
            }
        }

        assert!(proc_evs == vec![GameEvent::Move {id: ID::new_with_value(1), orig: (0,0), dst: (1,1)}],
                "Processed Events mismatch");
    }
}
