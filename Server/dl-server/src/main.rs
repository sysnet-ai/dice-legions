use std::collections::{HashMap};

mod game;

use game::components::*;
use game::resources::*;
use game::systems::*;
use game::conf_random::*; 


#[path = "server/server_actions.rs"]
mod server;
use server::*;

struct GameController;

impl GameController
{
    pub fn new() -> GameController
    {
        GameController { }
    }

    pub fn init(&mut self, map: Map<ID>, objs: HashMap<ID, Object>) -> GameState
    {
        let mut game_state = GameState::new(map, objs);

        self.advance_phase(&mut game_state, None); 

        game_state
    }

    //TODO: This could be done pretty with Enums
    pub fn advance_phase(&self, game_state: &mut GameState, actions: Option<&Vec<GameAction>>) -> (Option<Vec<GameAction>>, Option<Vec<GameEvent>>)
    {
        match game_state.phase
        {
            GamePhase::GameStart =>
            {
                ObjectInitializer::initialize(game_state);
                game_state.phase = GamePhase::GatherActions;
                assert!(actions.is_none(), "Actions sent on wrong phase");
                (None, None)
            },
            GamePhase::GatherActions =>
            {
                let available_actions = ActionGatherer::available_actions(game_state, &game_state.cur_player);
                game_state.phase = GamePhase::ProcessActions;
                assert!(actions.is_none(), "Actions sent on wrong phase");
                (Some(available_actions), None)
            },
            GamePhase::ProcessActions =>
            {
                assert!(actions.is_some(), "No Actions sent for Process Actions phase");

                let mut all_evts = vec![];
                let valid_actions:Vec<_> = actions.unwrap().iter().filter(|act| ActionValidator::validate_action(game_state, act)).collect();

                valid_actions.iter()
                             .for_each(|act| {
                                  let ev_list = ActionProcessor::process_action(game_state, act);
                                  let mut proc_events = EventProcessor::apply_events(game_state, ev_list);
                                  all_evts.append(&mut proc_events);
                              });

                game_state.phase = GamePhase::GatherActions;
                game_state.cur_player = game_state.cur_player.other();

                (None,  Some(all_evts))
            }
        }
    }
}

#[allow(dead_code)]
fn not_main() 
{
    use std::io::{stdin, stdout, Write};

    let mut gc = GameController::new();
    let map = Map::<ID>::new_with_dimensions((10, 10));
    let mut objs = HashMap::<ID, Object>::new();

    let components = vec![ Component::Movable { pos: (0, 0), max_speed: 2 },
                           Component::Attackable { max_health: 2,
                                                   attacker_dice: vec![DiceType::Smash, DiceType::Smash],
                                                   defender_dice: vec![DiceType::Smash, DiceType::Chain] } ];
    let obj = Object::new_with_components(ID::new_with_value(1), Owner::Player1, components);
    objs.insert(obj.id, obj);

    let components_2 = vec![ Component::Movable { pos: (2, 2), max_speed: 2 },
                             Component::Attackable { max_health: 2,
                                                     attacker_dice: vec![DiceType::Smash, DiceType::Chain],
                                                     defender_dice: vec![DiceType::Smash, DiceType::Chain] } ];
    let obj_2 = Object::new_with_components(ID::new_with_value(2), Owner::Player2, components_2);
    objs.insert(obj_2.id, obj_2);

    let mut game_state = gc.init(map, objs);
    loop
    {
        let actions = gc.advance_phase(&mut game_state, None).0.unwrap();
        print!("Available Actions: {:?}", actions);

        let mut s = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");

        s.pop();
        let action_inx: usize = s.parse().unwrap();

        let evts = gc.advance_phase(&mut game_state, Some(&vec![actions[action_inx]])).1.unwrap();

        print!("Caused Evts {:?}", evts);

        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
    }
}


extern crate futures;
extern crate tokio;
extern crate websocket;

use std::fmt::Debug;

use websocket::r#async::Server;
use websocket::message::{Message, OwnedMessage};
use websocket::server::InvalidConnection;

//TODO: These are not necessary, can change main to remove
use futures::{Future, Sink, Stream};
use tokio::runtime::TaskExecutor;
//TODO: Arrange this in a way that it's sane?
//
fn main()
{
	let mut runtime = tokio::runtime::Builder::new().build().unwrap();
	let reactor = runtime.reactor().clone();
	let executor = runtime.executor();
	// bind to the server
	let server = Server::bind("127.0.0.1:8080", &reactor).unwrap();

	// time to build the server's future
	// this will be a struct containing everything the server is going to do

	// a stream of incoming connections
	let f = server
		.incoming()
		// we don't wanna save the stream if it drops
		.map_err(|InvalidConnection { error, .. }| error)
		.for_each(move |(upgrade, addr)| {
			println!("Got a connection from: {}", addr);

			let f = upgrade
				.accept()
				.and_then(|(s, _)| s.send(Message::text("{\"EventType\": \"ConnectionSuccesful\"}").into()))
				.and_then(|s| {
					let (sink, stream) = s.split();
					stream
						.take_while(|m| Ok(!m.is_close()))
						.filter_map(|m| {
							println!("Message from Client: {:?}", m);
							match m {
								OwnedMessage::Ping(p) => Some(OwnedMessage::Pong(p)),
								OwnedMessage::Pong(_) => None,
								OwnedMessage::Text(t) => {
                                    let action_res = serde_json::from_str(&t);
                                    if action_res.is_ok()
                                    {
                                        match action_res.unwrap()
                                        {
                                            ServerAction::NewGame => {
                                                //
                                                let mut gc = GameController::new();
                                                let map = Map::<ID>::new_with_dimensions((10, 10));
                                                let mut objs = HashMap::<ID, Object>::new();

                                                let components = vec![ Component::Movable { pos: (0, 0), max_speed: 2 },
                                                                       Component::Attackable { max_health: 2,
                                                                                               attacker_dice: vec![DiceType::Smash, DiceType::Smash],
                                                                                               defender_dice: vec![DiceType::Smash, DiceType::Chain] } ];
                                                let obj = Object::new_with_components(ID::new_with_value(1), Owner::Player1, components);
                                                objs.insert(obj.id, obj);

                                                let components_2 = vec![ Component::Movable { pos: (2, 2), max_speed: 2 },
                                                                         Component::Attackable { max_health: 2,
                                                                                                 attacker_dice: vec![DiceType::Smash, DiceType::Chain],
                                                                                                 defender_dice: vec![DiceType::Smash, DiceType::Chain] } ];
                                                let obj_2 = Object::new_with_components(ID::new_with_value(2), Owner::Player2, components_2);
                                                objs.insert(obj_2.id, obj_2);

                                                let mut game_state = gc.init(map, objs);
                                                
                                                let new_game = ServerEvent::NewGameCreated { game_state: &game_state };
                                                Some(OwnedMessage::Text(serde_json::to_string(&new_game).unwrap()))
                                            }
                                        }
                                    }
                                    else
                                    {
                                        //TODO: Real errors
                                        Some(OwnedMessage::Text("Error Parsing Message".to_string()))
                                    }
                                },
                                _ => None
							}
						})
						.forward(sink)
						.and_then(|(_, sink)| sink.send(OwnedMessage::Close(None)))
				});

			spawn_future(f, "Client Status", &executor);
			Ok(())
		});

	runtime.block_on(f).unwrap();
}

fn spawn_future<F, I, E>(f: F, desc: &'static str, executor: &TaskExecutor)
where
	F: Future<Item = I, Error = E> + 'static + Send,
	E: Debug,
{
	executor.spawn(
		f.map_err(move |e| println!("{}: '{:?}'", desc, e))
			.map(move |_| println!("{}: Finished.", desc)),
	);
}
