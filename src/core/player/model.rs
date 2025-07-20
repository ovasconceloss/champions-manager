use uuid::Uuid;
use rand::{rngs::ThreadRng, Rng};
use crate::core::player::positions::Position;

#[derive(Debug)]
pub struct Player {
  pub uuid: Uuid,
  pub team: Uuid,
  pub name: String,
  pub overall: i32,
  pub position: Position,
}

impl Player {
  pub fn new(name: String, team_uuid: Uuid) -> Self {
    let mut rng: ThreadRng = rand::rng();

    let position: i32 = rng.random_range(0..3);
    let overall: i32 = rng.random_range(50..99);
    
    Self {
      uuid: Uuid::new_v4(),
      team: team_uuid,
      name: name,
      overall: overall,
      position: match position {
        0 => Position::Attack,
        1 => Position::Defense,
        2 => Position::Middlefier,
        _ => Position::Attack
      }
    }
  }
}