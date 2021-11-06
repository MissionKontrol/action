use action::*;
use std::{fmt::Debug, thread};

fn main() {
    let mut player1 = Actor::new( 0,0,10, 20, 6, 6);
    let mut player2= Actor::new( 1,1,10, 20, 6, 6);

    let (stdout_writer, sender) = PRINTER::new(action::ProcessorType::Stdout);
    player1 = player1.bind_channel(1,sender.clone());
    player2 = player2.bind_channel(1,sender.clone());

    let mut actor_list: ActorList = vec![player1,player2];
    let order_of_appearance = actor_list.clone();

    // let stdout_writer_thread = thread::Builder::new().
    //     name("stdout writer".to_string()).
    //     spawn(move || {
    //         loop {
    //             stdout_writer.read_channel();
    //         }
    //     });

    while !actor_list.done() {
        for actor in &order_of_appearance {
            if is_alive(actor_list.get_actor(&actor.get_id())){
                let mut action_state: Action = actor.get_action(&actor_list);
                println!("{}",actor.reporting_active());
            
                action_state = action_state.execute_action();

                actor_list = action_state.get_actor_list().to_owned();
                stdout_writer.read_channel();
            }
        }
    }
}

fn is_alive(actor: Option<Actor>) -> bool {
    match actor {
        Some(actor) => actor.is_alive(),
        None => false,
    }
}