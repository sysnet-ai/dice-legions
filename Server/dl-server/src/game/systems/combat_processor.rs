use crate::game::components::*;
use crate::game::conf_random::*;
use crate::game::systems::GameState;

use std::cmp;
use std::collections::HashSet;
use std::convert::TryInto;


#[derive(Debug)]
struct Chain
{
    dice_type: DiceType,
    chain: u8
}
#[derive(Debug)]
struct CombatState
{
    hits: Vec<GameEvent>,
    current_chain: Chain, 
    chain_effects: HashSet<DiceType>,
}

impl CombatState
{
    fn new() -> CombatState
    {
        CombatState
        {
            hits: vec![],
            current_chain: Chain { dice_type: DiceType::Smash, chain: 0 }, 
            chain_effects: HashSet::new(),
        }
    }
}

const CHAIN_SIZE:u8 = 3; // This could be a configurable value for CombatProcessor?

pub struct CombatProcessor;
impl CombatProcessor
{
    pub fn process_combat(game_state: &mut GameState, attacker_id: &ID, target_id: &ID) -> Vec<GameEvent>
    {
        //
        let attacker = game_state.objects.get(&attacker_id).unwrap().components.get(&ComponentID::AttackableID).unwrap(); 
        let target   = game_state.objects.get(&target_id).unwrap().components.get(&ComponentID::AttackableID).unwrap(); 

        let ctx = &mut game_state.rand_ctx;

        let mut attack_rolls:Vec<_> = if let Component::Attackable { max_health: _, attacker_dice, defender_dice: _ } = attacker
                                      {
                                          attacker_dice.iter().map(|&dt| DiceRoll::new_with_value(dt, ctx.gen_range::<i8>(1, 6))).collect()
                                      }
                                      else
                                      {
                                          panic!("No Attackable in Attacker");
                                      };
                                      
        let mut def_rolls:Vec<_> = if let Component::Attackable { max_health: _, attacker_dice: _, defender_dice } = target 
                                   {
                                       defender_dice.iter().map(|&dt| DiceRoll::new_with_value(dt, ctx.gen_range::<i8>(1, 6))).collect()
                                   }
                                   else
                                   {
                                       panic!("No Attackable in Target");
                                   };

        // TODO: MAGIC NUMBERS!
        let chain_adds:usize = attack_rolls.iter().filter(|r| DiceType::Chain == r.dice_type && r.value >= 5).collect::<Vec<&DiceRoll>>().len();
        for _ in 0..cmp::min(3, chain_adds)
        {
            attack_rolls.push(DiceRoll::new_with_value(DiceType::Chain, ctx.gen_range::<i8>(1, 6)));
        }

        let mut cs = CombatState::new();
        Self::process_rolls(&mut attack_rolls, &mut def_rolls, &mut cs);

        //TODO: Overwhelm, Defenseless, Counter...


        let mut combat_evs = vec![GameEvent::Combat { attacker_id: *attacker_id, target_id: *target_id }];
        
        combat_evs.append(&mut Self::generate_combat_events(attacker_id, target_id, &attack_rolls, &cs));
        combat_evs.append(&mut cs.hits);

        vec![GameEvent::EventContainer { contained_events: combat_evs }]
    }


    fn process_rolls(attack_rolls: &mut Vec<DiceRoll>, def_rolls: &mut Vec<DiceRoll>, cs: &mut CombatState)
    {
        attack_rolls.sort_by(|a, b| b.cmp(&a));

        let mut to_revisit = vec![]; // Revisit to allow more chains to be completed before using more dice of the same type


        //TODO: Ugly code dup
        for att in attack_rolls
        {
            if cs.current_chain.dice_type != att.dice_type
            {
                cs.current_chain = Chain { dice_type: att.dice_type, chain: 0 };
            }

            Self::find_best_hit(&att, def_rolls, cs, Some(&mut to_revisit));

            if cs.current_chain.chain >= CHAIN_SIZE
            {
                cs.chain_effects.insert(cs.current_chain.dice_type);
            }
        }

        for att in to_revisit
        {
            if cs.current_chain.dice_type != att.dice_type
            {
                cs.current_chain = Chain { dice_type: att.dice_type, chain: 0 };
            }

            Self::find_best_hit(&att, def_rolls, cs, None);

            if cs.current_chain.chain >= CHAIN_SIZE
            {
                cs.chain_effects.insert(cs.current_chain.dice_type);
            }
        }
    }

    fn generate_combat_events(attacker_id: &ID, target_id: &ID, attack_rolls: &Vec<DiceRoll>, cs: &CombatState) -> Vec<GameEvent>
    {
        let mut combat_evs = vec![];
        // TODO: MAGIC NUMBERS!
        for hit in cs.hits.iter()
        {
            if let GameEvent::Hit { attacker_dice, defender_dice } = hit
            {
                match attacker_dice.dice_type
                {
                    DiceType::Smash => {
                        combat_evs.push(GameEvent::StatDelta { id: *target_id, by: -2,  stat: StatID::Health });
                    }
                    DiceType::Trick => {
                        combat_evs.push(GameEvent::StatDelta { id: *target_id, by: -1,  stat: StatID::Health });
                    }
                    DiceType::Chain => {
                        if attacker_dice.value >= 4
                        {
                            combat_evs.push(GameEvent::StatDelta { id: *target_id, by: -1,  stat: StatID::Health });
                        }
                    }
                }
            }
        }

        for chain_type in cs.chain_effects.iter()
        {
            match chain_type
            {
                DiceType::Smash => {
                    // Add event Smash Chain - This is a stun, whtever that means
                }
                DiceType::Trick => {
                    // Add event Trick Chain, no idea what I'll do with this...
                }
                DiceType::Chain => {
                    // Add event Chain Chain // Moon Moon!
                    let chain_ct:usize = attack_rolls.iter().filter(|r| DiceType::Chain == r.dice_type).collect::<Vec<&DiceRoll>>().len();
                    combat_evs.push(GameEvent::StatDelta { id: *target_id, by: ((chain_ct + 1) / 2).try_into().unwrap(), stat: StatID::Health }); 
                }
            }
        }

        combat_evs
    }

    // Calculate Hits
    fn find_best_hit(roll:&DiceRoll, defense_rolls:&mut Vec<DiceRoll>, combat_state: &mut CombatState,
                     attacks_to_revisit: Option<&mut Vec<DiceRoll>>)
    {
        if combat_state.current_chain.chain == CHAIN_SIZE &&
           combat_state.current_chain.dice_type == roll.dice_type &&
           attacks_to_revisit.is_some() 
        {
            attacks_to_revisit.unwrap().push(*roll);
            return;
        }

        defense_rolls.sort_by(|a,b| b.value_as_seen_by(&roll).cmp(&a.value_as_seen_by(&roll)));

        for inx in 0..defense_rolls.len()
        {
            let def = defense_rolls[inx];
            if def.value_as_seen_by(&roll) < roll.value
            {
                combat_state.hits.push(GameEvent::Hit { attacker_dice: *roll, defender_dice: def });
                combat_state.current_chain.chain += 1;
                defense_rolls.remove(inx);
                break;
            }
        }
    }
}

#[cfg(test)]
mod test
{
    use super::*;

    #[test]
    fn test_find_hit()
    {
        let mut def_rolls = vec![(DiceType::Smash, 5), (DiceType::Trick, 5)]
                            .iter()
                            .map(|(t, v)| DiceRoll { dice_type: *t, value: *v })
                            .collect();

        let mut cs = CombatState::new();

        let att_roll = DiceRoll { dice_type: DiceType::Smash, value: 5 };  

        CombatProcessor::find_best_hit(&att_roll, &mut def_rolls, &mut cs, None);

        assert!(cs.hits[0] == GameEvent::Hit { attacker_dice: DiceRoll { dice_type: DiceType::Smash, value: 5 },
                                               defender_dice: DiceRoll { dice_type: DiceType::Trick, value: 5 } });

        assert!(cs.hits.len() == 1);
    }

    //TODO: Exhaustive?
    #[test]
    fn test_proc_rolls()
    {
        let mut att_rolls = vec![(DiceType::Smash, 6), (DiceType::Trick, 1), (DiceType::Smash, 4),
                                 (DiceType::Smash, 5), (DiceType::Smash, 4), (DiceType::Smash, 4)]
                            .iter()
                            .map(|(t, v)| DiceRoll { dice_type: *t, value: *v })
                            .collect();
        let mut def_rolls = vec![(DiceType::Smash, 3), (DiceType::Chain, 3), (DiceType::Smash, 3),
                                 (DiceType::Smash, 3), (DiceType::Smash, 6)]
                            .iter()
                            .map(|(t, v)| DiceRoll { dice_type: *t, value: *v })
                            .collect();

        let mut cs = CombatState::new(); 

        CombatProcessor::process_rolls(&mut att_rolls, &mut def_rolls, &mut cs);

        // TODO:  Check hit correctness

        assert!(cs.hits.len() == 4, "Incorrect amount of landed hits {:?}", cs.hits);
    }
}
