use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

use crate::ecs_lite::components::*;
use crate::ecs_lite::resources::*;
use crate::ecs_lite::systems::GameState;

pub struct ActionGatherer;
impl ActionGatherer
{
    pub fn available_actions(game_state: &GameState, for_player:&Owner) -> Vec<GameAction>
    {
        game_state
            .objects
            .iter()
            .filter(|(_, obj)| obj.owner == *for_player)
            .map(   |(_, obj)| 
                obj.components
                   .iter()
                   .map(|(_, comp)|
                        match comp
                        {
                            Component::Movable { pos, max_speed: _ } =>
                            {
                                let mut actions = Vec::<GameAction>::new();

                                let mut to_visit = vec![*pos];
                                let mut visited = HashSet::new();
                                let mut added = HashSet::new();

                                let threatened = ActionGatherer::get_threatened_positions(&game_state, &for_player);
                                let speed = obj.stats.get(&StatID::Speed).unwrap(); 

                                while !to_visit.is_empty()
                                {
                                    let visiting = to_visit.pop().unwrap();
                                    visited.insert(visiting);

                                    if !threatened.contains_key(&visiting)
                                    {
                                        for p in game_state.map.positions_within(&visiting, &1)
                                        {
                                            if !visited.contains(&p) &&
                                               game_state.map.manhattan_distance(pos, &p) < (*speed).try_into().unwrap()
                                            {
                                                to_visit.push(p);
                                            }

                                            if !added.contains(&p)
                                            {
                                                added.insert(p);

                                                match threatened.get(&p)
                                                {
                                                    Some(objs_attacking_p) =>
                                                    {
                                                        if obj.has_component(&ComponentID::AttackableID)
                                                        {
                                                            for &&tgt in objs_attacking_p
                                                            {
                                                                actions.push(GameAction::AttackOn { id: obj.id, target: tgt, orig: *pos, dst: p });
                                                            }
                                                        }
                                                    },
                                                    None =>
                                                    {
                                                        if game_state.map.get_id_at(&p).is_none()
                                                        {
                                                            actions.push(GameAction::MoveTo { id: obj.id, orig: *pos, dst: p });
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }

                                actions
                            },
                            Component::Attackable { max_health: _ } =>
                            {
                                Vec::<GameAction>::new()
                            }
                        })
                   .flatten() // Make all the actions for this object one list
                   .collect::<Vec<_>>())
            .flatten() // Make all the actions for all objects into one list
            .collect()
    }

    pub fn get_threatened_positions<'a>(game_state:&'a GameState, to_player:&Owner) -> HashMap<GridPosition, Vec<&'a ID>>
    {
        let mut threatened: HashMap::<GridPosition, Vec<&ID>> = HashMap::new();
        game_state
                    .objects
                    .iter()
                    .filter(  |(_, obj)| obj.owner != *to_player)
                    .for_each(|(_, obj)|
                        if obj.has_component(&ComponentID::AttackableID)
                        {
                            if let Some(Component::Movable { pos, max_speed: _ }) = obj.components.get(&ComponentID::MovableID)
                            {
                                let adjacents = game_state.map.positions_within(pos, &1);
                                adjacents.iter()
                                         .filter(|&&t_pos| t_pos != *pos)
                                         .for_each(|t_pos|
                                            match threatened.get_mut(t_pos)
                                            {
                                                Some(list) =>
                                                {
                                                    list.push(&obj.id);
                                                }
                                                None =>
                                                {
                                                    threatened.insert(*t_pos, vec![&obj.id]);
                                                }
                                            });
                            }
                        });

        threatened
    }
}


#[cfg(test)]
mod test
{
    use super::*;
    use super::super::ObjectInitializer;

    #[test]
    fn generate_moves()
    {
        let mut map = Map::<ID>::new_with_dimensions((10, 10));
        let mut objs = HashMap::<ID, Object>::new(); 

        let components = vec![ Component::Movable { pos: (0, 0), max_speed: 3 } ];
        let obj = Object::new_with_components(ID::new_with_value(1), Owner::Player1, components);
        objs.insert(obj.id, obj);

        let components_2 = vec![ Component::Movable { pos: (1, 1), max_speed: 3 } ];
        let obj_2 = Object::new_with_components(ID::new_with_value(2), Owner::Player2, components_2);
        objs.insert(obj_2.id, obj_2);

        let mut game_state = GameState { map: &mut map, objects: &mut objs };

        ObjectInitializer::initialize(&mut game_state);

        let acts = ActionGatherer::available_actions(&game_state, &Owner::Player1);

        assert!(!acts.is_empty(), "No Move actions generated during test");

        let mut poss: HashSet<GridPosition> = [/****/ (0,1), (0,2), (0,3),
                                               (1,0), (1,2), // (1,1) is occupied
                                               (2,0), (2,1),
                                               (3,0)].iter().cloned().collect();
        for a in acts
        {
            if let GameAction::MoveTo { id, orig, dst } = a 
            {
                assert!(id == ID::new_with_value(1), "Incorrect ID for MoveTo Action");
                assert!(orig == (0,0), "Incorrect origin for MoveTo Action");
                poss.get(&dst).unwrap();
                poss.remove(&dst);
            }
            else
            {
                assert!(false, "Incorrect Action type returned from MoveTo scenario");
            }
        }

        assert!(poss.is_empty(), "Spurios destination values in MoveTo actions");
    }

    #[test]
    fn generate_moves_and_attacks()
    {
        let mut map = Map::<ID>::new_with_dimensions((10, 10));
        let mut objs = HashMap::<ID, Object>::new(); 

        let components = vec![ Component::Movable { pos: (0, 0), max_speed: 3 },
                               Component::Attackable { max_health: 1 } ];
        let obj = Object::new_with_components(ID::new_with_value(1), Owner::Player1, components);
        objs.insert(obj.id, obj);

        let components_2 = vec![ Component::Movable { pos: (1, 2), max_speed: 3 },
                                 Component::Attackable { max_health: 1 } ];
        let obj_2 = Object::new_with_components(ID::new_with_value(2), Owner::Player2, components_2);
        objs.insert(obj_2.id, obj_2);

        let components_3 = vec![ Component::Movable { pos: (0, 3), max_speed: 3 },
                                 Component::Attackable { max_health: 1 } ];
        let obj_3 = Object::new_with_components(ID::new_with_value(3), Owner::Player2, components_3);
        objs.insert(obj_3.id, obj_3);

        let mut game_state = GameState { map: &mut map, objects: &mut objs };

        ObjectInitializer::initialize(&mut game_state);
        let acts = ActionGatherer::available_actions(&game_state, &Owner::Player1);

        assert!(!acts.is_empty(), "No Move + Attack actions generated during test");

        let mut moves: HashSet<GridPosition> = [(0,1), (1,0), (2,0), (3,0), (2,1)].iter().cloned().collect();
        let mut attacks: HashSet<(GridPosition, ID)> = [((0,2), ID(2)), ((1,1), ID(2)), ((0,2), ID(3))].iter().cloned().collect();

        for a in acts
        {
            match a
            {
                GameAction::MoveTo { id, orig, dst } =>
                {
                    assert!(id == ID::new_with_value(1), "Incorrect ID for MoveTo Action");
                    assert!(orig == (0,0), "Incorrect origin for MoveTo Action");
                    moves.get(&dst).unwrap();
                    moves.remove(&dst);
                },
                GameAction::AttackOn { id, target, orig, dst } =>
                {
                    assert!(id == ID::new_with_value(1),     "Incorrect ID for AttackOn Action");
                    assert!(orig == (0,0), "Incorrect origin for AttackOn Action");
                    attacks.get(&(dst, target)).unwrap();
                    attacks.remove(&(dst, target));
                }
            }
        }

        assert!(moves.is_empty(),   "Spurios destination values in MoveTo actions");
        assert!(attacks.is_empty(), "Spurios destination values in AttackOn actions");
    }
}
