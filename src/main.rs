use action::*;

fn main() {
    let action: Action = action::ACTION::new();
    println!("{}",action.get_action());
    println!("{}",action.get_state());

    let action: Action = action.execute_action();
    println!("{}",action.get_action());
    println!("{}",action.get_state());
}
