use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag="ActionType")]
pub enum ServerAction
{
    NewGame,
}

//TODO: I fucked up the file org and can't see GameState from here
#[derive(Serialize, Debug)]
#[serde(tag="EventType")]
pub enum ServerEvent<'a, T> where T: Serialize
{
    NewGameCreated { game_state: &'a T},
}




