pub type Action = Box<dyn ActionState>;
pub type ACTION = dyn ActionState;

impl ACTION {
    pub fn new() -> Action {
        let actor_list = vec!["actor_list".to_string()];
        Box::new(ActionReady{ actor_list, readied_action: ActionType{}})
    }
}

pub trait ActionState {
    fn get_state(&self) -> String;
    fn get_actor_list(&self) -> Vec<String>;
    fn get_action(&self) -> String;
    fn execute_action(&self) -> Action;
}

#[derive(Clone)]
pub struct ActionReady {
    actor_list: Vec<String>,
    readied_action: ActionType,
}

impl ActionState for ActionReady {
    fn get_state(&self) -> String {
        "Ready".to_string()
    }

    fn get_actor_list(&self) -> Vec<String> {
        self.actor_list.clone()
    }

    fn get_action(&self) -> String {
        "Tots prepared".to_string()
    }

    fn execute_action(&self) -> Action {
        Box::new(ActionComplete{actor_list: self.actor_list.clone()})

    }
}

#[derive(Clone)]
pub struct ActionComplete {
    actor_list: Vec<String>,
}

impl  ActionState for ActionComplete {
    fn get_state(&self) -> String {
        "Complete".to_string()
    }
    fn get_actor_list(&self) -> Vec<String> {
        self.actor_list.clone()
    }

    fn get_action(&self) -> String {
        "Tots finished".to_string()
    }

    fn execute_action(&self) ->  Action {
        Box::new(self.clone())
    }
}

#[derive(Clone, Copy)]
struct ActionType {}

struct Attack {

}

impl Attack {
    fn execute_attack(&self) {

    }

}