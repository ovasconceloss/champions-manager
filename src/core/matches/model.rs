use uuid::Uuid;

#[derive(Debug)]
pub struct Matches {
  pub home_uuid: Uuid,
  pub away_uuid: Uuid,
  pub home_score: i32,
  pub away_score: i32
}

impl Matches {
  pub fn new(home_uuid: Uuid, away_uuid: Uuid) -> Self {
    Self { home_uuid: home_uuid, away_uuid: away_uuid, home_score: 0, away_score: 0 }
  }
}