use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Team {
  pub uuid: Uuid,
  pub name: String,
  pub reputation: i32,
  pub abbreviation: String,
}

#[derive(Debug, Clone)]
pub struct TeamStatus {
  pub attack_overall: i32,
  pub defense_overall: i32,
  pub midfield_overall: i32
}

impl Team {
  pub fn new(uuid: Uuid, name: String, reputation: i32, abbreviation: String) -> Self {
    Self { 
      uuid: uuid, 
      name: name, 
      reputation: reputation, 
      abbreviation: abbreviation 
    }
  }
}

impl TeamStatus {
  pub fn new_status(attack: i32, defense: i32, midfield: i32) -> Self {
    Self { attack_overall: attack, defense_overall: defense, midfield_overall: midfield }
  }
}