use rand::Rng;

pub type Action = Box<ACTION>;
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
    actor_list: ActorList,
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

#[derive(Debug, Clone)]
pub enum ActionType {
    Attack(Attack),
    Dodge,
    None,
}

#[derive(Debug, Clone)]
pub struct Attack {
    target: u8,
    attack_die: u8,
    damage_die: u8,
}

trait Actions {
    fn execute(&self, current_actors: ActorList) -> ActorList;
}

impl Actions for Attack {
    fn execute(&self, mut current_actors: ActorList) -> ActorList {
        let result = Attack::roll_attack();
        let mut target = current_actors.remove(&self.target).unwrap();  // wrong by id of actor
        if target.does_this_hit(result) {
            let damage = Attack::roll_damage(&self.damage_die);
            target = target.take_damage(damage);
        }

        if target.is_alive() { 
            current_actors.insert(target.id,target);
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

pub type ActorList = HashMap<u8,Actor>;

pub trait ActorListActions {
    fn get_actor(&self, id: &u8) -> Option<Actor>;
    fn done(&self) -> bool;
    fn bind_channel(self, channel: Sender<String>, receiver_id: u8) -> Self;
}

impl ActorListActions for ActorList {
    fn get_actor(&self, id: &u8) -> Option<Actor> {
        let actor = self.get(id);
        match actor {
            Some(actor) => Some(actor.to_owned()),
            None => None
        }
    }

    fn done(&self) -> bool {
        let team_a_size = &self.iter().fold(0_u8,|acc, actor| if actor.1.team_id == 0 { acc + 1} else { acc });
        let team_b_size = &self.iter().fold(0_u8,|acc, actor| if actor.1.team_id == 1 { acc + 1} else { acc });
        if team_a_size == &0 || team_b_size == &0 { return true }
        else { false }
    }

    fn bind_channel(self, sender: Sender<String>, reciever_id: u8) -> Self {
        let mut actor_list: ActorList = HashMap::new();

        for (id, player) in self {
            let player = player.bind_channel(reciever_id,sender.clone());
            actor_list.insert(id, player);
        }
        actor_list
    }
}

#[derive(Debug, Clone)]
pub struct Actor {
    id: u8,
    team_id: u8,
    armour_class: u8,
    hit_points: u8,
    hit_die: u8,
    damage_die: u8,
    report_bindings: HashMap<u8,Sender<String>>,
}

impl Actor {
    pub fn new(id: u8, team_id: u8, armour_class: u8, hit_points: u8, hit_die: u8, damage_die: u8) -> Self { 
        Self { id, team_id, armour_class, hit_points, hit_die, damage_die, report_bindings: HashMap::new()}}

    pub fn get_action(&self, actor_list: &ActorList) -> Action {
        let action = self.decide_action(actor_list);

        if self.reporting_active(){
            self.send_report(format!("{} decides to {:?}", self.id, action));
        }

        Box::new(ActionReady{ actor_list: actor_list.clone(), readied_action: action})
    }

    pub fn does_this_hit(&self, attack_roll: u8) -> bool {
        if attack_roll as u8 > self.armour_class { 
            if self.reporting_active() {
                self.send_report(format!("{} Hits",attack_roll));
            }
            return true 
        }
        if self.reporting_active() {
            self.send_report(format!("{} Misses", attack_roll));
        }
        return false
    }

    pub fn take_damage(mut self, damage: u8) -> Actor {
        let remaining_points = self.hit_points as i16 - damage as i16;
        if remaining_points <= 0 {
            if self.reporting_active() {
                self.send_report(format!("{} takes {} dies.", self.id, damage));
            }
            self.hit_points = 0;
            self = self.unbind_channel(1);
        }
        else {
            self.hit_points = remaining_points as u8; 
            if self.reporting_active() {
                self.send_report(format!("{} takes {} and survives on {}.", self.id, damage, self.hit_points));
            } 
        }
        
        self
    }

    pub fn is_alive(&self) -> bool {
        if self.hit_points > 0 { return true }
        false
    }

    pub fn get_id(&self) -> u8 {
        self.id
    }

    fn select_target(&self, actor_list:  &ActorList) -> Option<u8> {
        let target_list = actor_list
            .iter()
            .filter(|actor| actor.1.team_id != self.team_id)
            .collect::<HashMap<&u8,&Actor>>();
   
        if let Some(target) = target_list.keys().next() {
            Some(**target)
        } 
        else { None }
    }

    fn decide_action(&self, actor_list: &ActorList) -> ActionType {
        if let Some(target) = self.select_target(actor_list) {
            ActionType::Attack(Attack{target, attack_die: self.damage_die, damage_die: self.damage_die})
        }
        else {
            ActionType::None
        }
    }
}

use std::{collections::HashMap, sync::mpsc::{Receiver,RecvError, Sender, channel}};

pub type Printer = Box<PRINTER>;
pub type PRINTER = dyn ReportProcessor + Send;

impl PRINTER{
    pub fn new(processor_type: ProcessorType) -> (Sender<String>, Printer) {
      let (sender, receiver):(Sender<String>, std::sync::mpsc::Receiver<_>) = channel();

        match processor_type {
            ProcessorType::Stdout  => (sender, Box::new(StdConsole {receiver}) as Printer),
            ProcessorType::Stderr  => (sender, Box::new(StdConsole {receiver})),
            ProcessorType::File(_) => (sender, Box::new(StdConsole {receiver})),
        }
    }
}

pub enum ProcessorType {
    Stdout,
    Stderr,
    File(String),
}

pub trait ReportProcessor {
    fn bind_channel(self, receiver: Receiver<String>);
    fn read_channel(&self) -> bool;
    fn flush_buffer(&self, buffer: String);
    fn receive(&self) -> Result<String, RecvError>;
}

struct StdConsole {
    receiver: Receiver<String>,
}

impl ReportProcessor for StdConsole {
    fn bind_channel(mut self, receiver: Receiver<String>) {
        self.receiver = receiver;
    }

    fn read_channel(&self) -> bool {
        let buffer = self.receiver.recv();
        match buffer {
            Ok(buffer) => { self.flush_buffer(buffer); true },
            Err(error) => { println!("StdConsole recieve error: {}", error); false },
        }
    }

    fn flush_buffer(&self, buffer: String){
        println!("{}",buffer);
    }

    fn receive(&self) -> Result<String, RecvError> {
        self.receiver.recv()
    }
}

pub trait Reporter {
    fn bind_channel(self, processor_id: u8, reporter: Sender<String>) -> Self;
    fn unbind_channel(self, processor_id: u8) -> Self;
    fn reporting_active(&self) -> bool;
    fn send_report(&self, buffer: String);
}

impl Reporter for Actor {
    fn bind_channel(mut self, processor_id: u8, sender: Sender<String>) -> Self {
        self.report_bindings.insert(processor_id, sender);
        self
    }

    fn unbind_channel(mut self, processor_id: u8) -> Self {
        self.report_bindings.remove(&processor_id);
        // println!("Binding {} removed", processor_id);
        self
    }

    fn reporting_active(&self) -> bool {
        if self.report_bindings.len() > 0 { true }
        else { false }
    }

    fn send_report(&self, buffer: String) {
        for (_, binding) in &self.report_bindings {
            if let Err(error) = binding.send(buffer.clone()){
                println!("Actor send error: {}", error);
            }
        }
    }
}