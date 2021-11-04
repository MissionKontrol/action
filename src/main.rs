use action::*;

fn main() {
    let player1 = Actor::new( 0,1,10, 20, 6, 6);
    let player2= Actor::new( 1,2,10, 20, 6, 6);

    let actor_list = vec![player1,player2];

    let action = actor_list[0].get_action(&actor_list);

    println!("{}",action.get_action());
    println!("{}",action.get_state());
    println!("{:?}",action.get_actor_list());

    let action: Action = action.execute_action();
    println!("{}",action.get_action());
    println!("{}",action.get_state());
    println!("{:?}",action.get_actor_list());

    let action = actor_list[1].get_action(&action.get_actor_list());

    println!("{}",action.get_action());
    println!("{}",action.get_state());
    println!("{:?}",action.get_actor_list());

    let action: Action = action.execute_action();
    println!("{}",action.get_action());
    println!("{}",action.get_state());
    println!("{:?}",action.get_actor_list());
}
