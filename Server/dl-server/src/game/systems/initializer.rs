use crate::game::components::*;
use crate::game::resources::*;
use crate::game::systems::GameState;

pub struct ObjectInitializer;
impl ObjectInitializer
{
    pub fn initialize(game_state: &mut GameState)
    {
        for (id, obj) in &mut game_state.objects.iter_mut()
        {
           if let Some(Component::Movable { pos, max_speed }) = obj.components.get(&ComponentID::MovableID) 
           {
               game_state.map.add_obj(*id, pos);
               obj.stats.insert(StatID::Speed, *max_speed);
           }

           if let Some(Component::Attackable { max_health }) = obj.components.get(&ComponentID::AttackableID) 
           {
               obj.stats.insert(StatID::Health, *max_health);
           }
        }
    }
}
