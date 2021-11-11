use action::*;
use std::{thread};
use std::sync::mpsc::{Sender};
use std::collections::HashMap;

fn main() {
    let actor_list = get_players();
    let (sender, stdout_writer): (Sender<String>, Printer)
        = PRINTER::new(action::ProcessorType::Stdout);

    let reciever_id = 0;
    let mut actor_list = actor_list.bind_channel(sender.clone(), reciever_id);

    let order_of_appearance = actor_list.clone();

    let _stdout_writer_thread = thread::Builder::new().
        name("stdout writer".to_string()).
        spawn(move || {
            loop {
                let buffer = stdout_writer.receive();
                match buffer {
                    Ok(buffer) => stdout_writer.flush_buffer(buffer),
                    Err(_) => {
                        break;
                    },
                }  
            }
        })
        .unwrap();

    for _ in 0..5 {
        let mut turn = 0_u32;
        let mut acts = 0_u32;

        actor_list = order_of_appearance.clone();
        while !actor_list.done() {
            turn +=1;
            for actor in &order_of_appearance {
                if is_alive(actor_list.get_actor(&actor.1.get_id())){
                    acts += 1;
                    let mut action_state: Action = actor.1.get_action(&actor_list);
                    action_state = action_state.execute_action();
                    actor_list = action_state.get_actor_list().to_owned();
                }
            }
        }

        let mut winners = String::new();
        for i in actor_list.keys(){
            winners = format!("{} {}", i, winners);
        }
        if let Err(error) =  sender.send(format!("Winner {} turns {}  acts {}", winners, turn, acts)){
            println!("Error sending winner: {}", error);
        }
    }
    
    drop(sender);

    // for actor in actor_list {
    //     actor.unbind_channel(1);
    // }

    // use std::{time};
    // let ten_millis = time::Duration::from_millis(10);
    // thread::sleep(ten_millis);
}

fn is_alive(actor: Option<Actor>) -> bool {
    match actor {
        Some(actor) => actor.is_alive(),
        None => false,
    }
}

fn get_players() -> HashMap<u8, Actor> {
    let player1 = Actor::new( 0,0,10, 20, 6, 6);
    let player2 = Actor::new( 1,1,10, 20, 6, 6);
    let player3 = Actor::new( 2,0,10, 20, 6, 6);
    let player4 = Actor::new( 3,1,10, 20, 6, 6);

    let mut actor_list: ActorList = HashMap::new();
    actor_list.insert(player1.get_id(), player1);
    actor_list.insert(player2.get_id(), player2);
    actor_list.insert(player3.get_id(), player3);
    actor_list.insert(player4.get_id(), player4);

    actor_list
}