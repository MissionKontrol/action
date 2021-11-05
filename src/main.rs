use action::*;

fn main() {
    let player1 = Actor::new( 0,0,10, 20, 6, 6);
    let player2= Actor::new( 1,1,10, 20, 6, 6);

    let mut actor_list: ActorList = vec![player1,player2];
    let order_of_appearance = actor_list.clone();

    while !actor_list.done() {
        for actor in &order_of_appearance {
            if is_alive(actor_list.get_actor(&actor.get_id())){
                let mut action_state: Action = actor.get_action(&actor_list);

                println!("{}",action_state.get_action());
                println!("{}",action_state.get_state());
                println!("{:?}",action_state.get_actor_list());
            
                action_state = action_state.execute_action();
                println!("{}",action_state.get_action());
                println!("{}",action_state.get_state());
                println!("{:?}",action_state.get_actor_list());
                actor_list = action_state.get_actor_list().to_owned();
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