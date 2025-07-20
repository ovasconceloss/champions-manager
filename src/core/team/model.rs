#[derive(Debug, Clone)]
pub struct Team {
  pub name: String,
  pub reputation: i32,
  pub abbreviation: String,
}

impl Team {
  pub fn new(name: String, reputation: i32, abbreviation: String) -> Self {
    Self { name: name, reputation: reputation, abbreviation: abbreviation }
  }
}