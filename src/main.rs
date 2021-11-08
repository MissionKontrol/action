use action::*;
use std::{thread};
use std::sync::mpsc::{Sender};

fn main() {
    let mut player1 = Actor::new( 0,0,10, 20, 6, 6);
    let mut player2 = Actor::new( 1,1,10, 20, 6, 6);
    // let mut player3 = Actor::new( 2,0,10, 20, 6, 6);
    // let mut player4 = Actor::new( 3,1,10, 20, 6, 6);

    let (sender, stdout_writer): (Sender<String>, Printer)
         = PRINTER::new(action::ProcessorType::Stdout);
    player1 = player1.bind_channel(1,sender.clone());
    player2 = player2.bind_channel(1,sender.clone());

    let mut actor_list: ActorList = vec![player1,player2];
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
                if is_alive(actor_list.get_actor(&actor.get_id())){
                    acts += 1;
                    let mut action_state: Action = actor.get_action(&actor_list);
                    action_state = action_state.execute_action();
                    actor_list = action_state.get_actor_list().to_owned();
                }
            }
        }
        if let Err(error) =  sender.send(format!("Winner {} turns {}  acts {}", actor_list[0].get_id(), turn, acts)){
            println!("Error sending winner: {}", error);
        }
    }
    



    drop(sender);

    for actor in actor_list {
        actor.unbind_channel(1);
    }

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