use super::components::*;

use std::convert::TryInto;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[allow(dead_code)]
pub enum Owner
{
    Player1,
    Player2,
    System
}

impl Owner
{
    pub fn other(&self) -> Owner
    {
        match self
        {
            Owner::Player1 => Owner::Player2,
            Owner::Player2 => Owner::Player1,
            Owner::System => Owner::System
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Object 
{
    pub id: ID,
    pub owner: Owner,
    pub components: HashMap<ComponentID, Component>,
    pub stats: HashMap<StatID, i32>,
}

impl Object
{
    pub fn new_with_components(id: ID, owner: Owner, components: Vec<Component>) -> Object
    {
        let mut res = Object
        {
            id: id,
            owner: owner,
            components: HashMap::new(), 
            stats: HashMap::new(),
        };

        for comp in components
        {
            res.add_component(comp);
        }

        res
    }

    pub fn add_component(&mut self, comp: Component)
    {
        let key = comp.get_component_id(); 
        self.components.insert(key, comp);
    }

    pub fn has_component(&self, comp_id: &ComponentID) -> bool
    {
        self.components.contains_key(comp_id)
    }

    pub fn get_stat_value(&self, id: &StatID) -> i32 
    {
        let v = self.stats.get(id).unwrap();
        // v = effects.get(id).modify(v);
        *v 
    }
}


#[derive(Serialize, Deserialize)]
pub struct Map<T>
{
    grid: Vec<Vec<T>>,
    empty_value: T
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl<T> Map<T> where T:Clone + Copy + Default + PartialEq
{
    pub fn new_with_dimensions(dims: (usize, usize)) -> Map<T>
    {
        assert!(dims.0 != 0 && dims.1 != 0, "Can't create a Map with dimensions 0 {:?}", dims);
        let mut grid = vec![];
        for i in 0..dims.0
        {
            grid.push(vec![T::default(); dims.1]);
        }

        Map
        {
            grid: grid,
            empty_value: T::default()
        }
    }

    pub fn get_id_at(&self, g_pos: &GridPosition) -> Option<T>
    {
        let v = self.grid[g_pos.0][g_pos.1];
        if v != self.empty_value
        {
            Some(v)
        }
        else
        {
            None
        }
    }

    pub fn is_occupied(&self, g_pos: &GridPosition) -> bool
    {
        self.get_id_at(g_pos).is_some()
    }

    pub fn move_obj(&mut self, start_pos: &GridPosition, end_pos: &GridPosition)
    {
        match self.get_id_at(start_pos)
        {
            Some(obj_id) =>
            {
                let ep_id = self.get_id_at(end_pos);
                if ep_id.is_some()
                {
                    self.remove_obj(&end_pos);
                }

                self.remove_obj(&start_pos);
                self.add_obj(obj_id, &end_pos);
            },
            None => assert!(false, "Trying to move from an empty position")
        }
    }

    pub fn add_obj(&mut self, new_id: T, at: &GridPosition)
    {
        assert!(!self.is_occupied(at), "Trying to add on a non-empty position");
        self.grid[at.0][at.1] = new_id; 
    }

    pub fn remove_obj(&mut self, at: &GridPosition)
    {
        assert!(self.is_occupied(at), "Trying to remove from an empty position");
        self.grid[at.0][at.1] = self.empty_value;
    }

    pub fn manhattan_distance(&self, from:&GridPosition, to:&GridPosition) -> usize
    {
        let i64_from:(i64, i64) = (from.0.try_into().unwrap(),
                                   from.1.try_into().unwrap());

        let i64_to:(i64, i64) = (to.0.try_into().unwrap(),
                                 to.1.try_into().unwrap());

        ((i64_from.0 - i64_to.0).abs() + (i64_from.1 - i64_to.1).abs()).try_into().unwrap()
    }

    pub fn positions_within(&self, from: &GridPosition, manh_dist: &usize) -> Vec<GridPosition>
    {
        let mut poss = vec![];

        //TODO: Some of this seems gross
        let s_manh_dist:i64     = (*manh_dist).try_into().unwrap();
        let i64_from:(i64, i64) = (from.0.try_into().unwrap(),
                                   from.1.try_into().unwrap());
        let i64_dims:(i64, i64) = (self.grid.len().try_into().unwrap(),
                                   self.grid[0].len().try_into().unwrap());

        for i in -s_manh_dist..s_manh_dist+1
        {
            for j in -s_manh_dist..s_manh_dist+1
            {
                if i.abs() + j.abs() <= s_manh_dist
                {
                    let nx = i64_from.0 + i;
                    let ny = i64_from.1 + j;

                    if 0 <= nx && nx <= i64_dims.0 && 
                       0 <= ny && ny <= i64_dims.1 
                    {
                        poss.push((nx.try_into().unwrap(), ny.try_into().unwrap()));
                    }
                }
            }
        }
        poss
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    #[should_panic]
    fn remove_empty_panic()
    {
        let mut m:Map<ID> = Map::new_with_dimensions((10, 10));
        m.remove_obj(&(0,0));
    }

    #[test]
    #[should_panic]
    fn add_occupied_panic()
    {
        let mut m:Map<ID> = Map::new_with_dimensions((10, 10));
        m.add_obj(ID::new_with_value(1), &(0,0));
        m.add_obj(ID::new_with_value(2), &(0,0));
    }

    #[test]
    fn add_and_remove()
    {
        let mut m:Map<ID> = Map::new_with_dimensions((10, 10));
        m.add_obj(ID::new_with_value(1), &(0,0));
        m.remove_obj(&(0,0));
        assert!(!m.is_occupied(&(0,0)), "Removing not working")
    }

    #[test]
    fn add_and_move()
    {
        let mut m:Map<ID> = Map::new_with_dimensions((10, 10));
        m.add_obj(ID::new_with_value(1), &(0,0));
        m.move_obj(&(0,0), &(1,1));

        assert!(!m.is_occupied(&(0,0)), "Moving not working - OBJ Still in original spot");

        match m.get_id_at(&(1,1))
        {
            Some(id) => assert!(id.0 == 1, "Moving not working - OBJ with incorrect id at destination"),
            None => assert!(false, "Moving not working - OBJ not in destination spot")
        }
    }

    #[test]
    fn add_and_move_to_occ()
    {
        let mut m:Map<ID> = Map::new_with_dimensions((10, 10));
        m.add_obj(ID::new_with_value(1), &(0,0));
        m.add_obj(ID::new_with_value(2), &(1,1));
        m.move_obj(&(0,0), &(1,1));

        assert!(!m.is_occupied(&(0,0)), "Moving not working - OBJ Still in original spot");

        match m.get_id_at(&(1,1))
        {
            Some(id) => assert!(id.0 == 1, "Moving not working - OBJ with incorrect id at destination"),
            None => assert!(false, "Moving not working - OBJ not in destination spot")
        }
    }
}
