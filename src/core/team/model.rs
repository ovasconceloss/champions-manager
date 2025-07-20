use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Team {
  pub uuid: Uuid,
  pub name: String,
  pub reputation: i32,
  pub abbreviation: String,
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