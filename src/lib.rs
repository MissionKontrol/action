use rand::Rng;

pub type Action = Box<dyn ActionState>;
pub type ACTION = dyn ActionState;

impl ACTION {
    pub fn new(actor_list: ActorList) -> Action {
        Box::new(ActionReady{ actor_list, readied_action: ActionType::Dodge})
    }
}

pub trait ActionState {
    fn get_state(&self) -> String;
    fn get_actor_list(&self) -> &ActorList;
    fn get_action(&self) -> String;
    fn execute_action(&self) -> Action;
}

#[derive(Clone)]
pub struct ActionReady {
    actor_list: Vec<Actor>,
    readied_action: ActionType,
}

impl ActionState for ActionReady {
    fn get_state(&self) -> String {
        "Ready".to_string()
    }

    fn get_actor_list(&self) -> &ActorList {
        &self.actor_list
    }

    fn get_action(&self) -> String {
        match &self.readied_action {
            ActionType::Attack(description) => format!("get ATTACK: {}",description.target),
            _ => "Other".to_string(),
        }
    }

    fn execute_action(&self) -> Action {
        if let ActionType::Attack(description) = &self.readied_action {
            let result = description.execute(self.actor_list.clone());
            return Box::new(ActionComplete{actor_list: result});
        }
        Box::new(ActionComplete{actor_list: self.actor_list.clone()})

    }
}

#[derive(Clone)]
pub struct ActionComplete {
    actor_list: ActorList,
}

impl  ActionState for ActionComplete {
    fn get_state(&self) -> String {
        "Complete".to_string()
    }

    fn get_actor_list(&self) -> &ActorList {
        &self.actor_list
    }

    fn get_action(&self) -> String {
        "Tots finished".to_string()
    }

    fn execute_action(&self) ->  Action {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
pub enum ActionType {
    Attack(Attack),
    Dodge,
    None,
}

#[derive(Debug, Clone)]
pub struct Attack {
    target: usize,
    attack_die: u8,
    damage_die: u8,
}

trait Actions {
    fn execute(&self, current_actors: ActorList) -> ActorList;
}

impl Actions for Attack {
    fn execute(&self, mut current_actors: ActorList) -> ActorList {
        let result = Attack::roll_attack();
        let mut target = current_actors.remove(self.target);
        print!("Does this {} hit? ",&result);
        if target.does_this_hit(result) {
            println!("Yes!");
            let damage = Attack::roll_damage(&self.damage_die);
            target = target.take_damage(damage);
        }

        if target.is_alive() { 
            current_actors.push(target);
        }
            current_actors
    }
}

impl Attack {
    fn roll_attack() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..=20) as u8
    }

    fn roll_damage(damage_die: &u8) -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen_range(1..= *damage_die) as u8
    }
}

pub type ActorList = Vec<Actor>;

pub trait ActorListActions {
    fn get_actor(&self, id: &usize) -> Option<Actor>;
    fn done(&self) -> bool;
}

impl ActorListActions for ActorList {
    fn get_actor(&self, id: &usize) -> Option<Actor> {
        match self.iter().find(|actor| actor.id == *id){
            Some(actor) => Some(actor.to_owned()),
            None => None
        }
    }

    fn done(&self) -> bool {
        let team_a_size = &self.iter().fold(0_u8,|acc, actor| if actor.team_id == 0 { acc + 1} else { acc });
        let team_b_size = &self.iter().fold(0_u8,|acc, actor| if actor.team_id == 1 { acc + 1} else { acc });
        println!("team A: {} team B: {}", team_a_size,team_b_size);
        if team_a_size == &0 || team_b_size == &0 { return true }
        else { false }
    }
}

#[derive(Debug, Clone)]
pub struct Actor {
    id: usize,
    team_id: u8,
    armour_class: u8,
    hit_points: u8,
    hit_die: u8,
    damage_die: u8,
}

impl Actor {
    pub fn new(id: usize, team_id: u8, armour_class: u8, hit_points: u8, hit_die: u8, damage_die: u8) -> Self { 
        Self { id, team_id, armour_class, hit_points, hit_die, damage_die } }

    pub fn get_action(&self, actor_list: &ActorList) -> Action {
        let action = self.decide_action(actor_list);
        Box::new(ActionReady{ actor_list: actor_list.clone(), readied_action: action})
    }

    pub fn does_this_hit(&self, attack_roll: u8) -> bool {
        if attack_roll as u8 > self.armour_class { return true }
        return false
    }

    pub fn take_damage(mut self, damage: u8) -> Actor {
        let remaining_points = self.hit_points as i16 - damage as i16;
        if remaining_points <= 0 {
            self.hit_points = 0;
        }
        else { self.hit_points = remaining_points as u8; }
        self
    }

    pub fn is_alive(&self) -> bool {
        if self.hit_points > 0 { return true }
        false
    }

    pub fn get_id(&self) -> usize {
        self.id
    }

    fn select_target(&self, actor_list:  &Vec<Actor>) -> Option<usize> {
        let target_list = actor_list.iter().filter(|x| x.team_id != self.team_id).collect::<Vec<&Actor>>();
        if target_list.len() > 0 { Some(target_list[0].id) }
        else { None }
    }

    fn decide_action(&self, actor_list: &Vec<Actor>) -> ActionType {
        if let Some(target) = self.select_target(actor_list) {
            ActionType::Attack(Attack{target, attack_die: self.damage_die, damage_die: self.damage_die})
        }
        else {
            ActionType::None
        }
    }
}