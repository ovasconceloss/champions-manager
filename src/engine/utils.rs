use rand::Rng;
use crate::core::player::{model::Player, positions::Position};

pub fn normalize_overall(value: i32, minimum: i32, maximum: i32) -> i32 {
  ((value - minimum) * 99) / (maximum - minimum)
}

pub fn calculate_team_strength(squad: &[Player]) -> (i32, i32, i32) {
  squad.iter().fold((0, 0, 0), |mut accumulator, player| {
    match player.position {
      Position::Attack => accumulator.0 += player.overall,
      Position::Defense => accumulator.1 += player.overall,
      Position::Middlefier => accumulator.2 += player.overall,
    }

    accumulator
  })
}

pub fn generate_poisson_variante(xg: f64) -> i32 {
  let mut rng = rand::rng();

  let l: f64 = (-xg).exp();
  let mut k: i32 = 0;
  let mut p: f64 = 1.0;

  loop {
    let u: f64 = rng.random_range(0.0..1.0);
    p = p * u;
    k = k + 1;
    if p <= l {
      return k - 1;
    }
  }
}