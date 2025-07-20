use rand::Rng;
use crate::core::{matches::model::Matches, player::{model::Player, positions::Position}, team::model::{Team, TeamStatus}};

const N_ATTACK: i32 = 3;
const N_DEFENSE: i32 = 4;
const N_MIDFIELD: i32 = 3;
const PLAYER_MIN_OVERALL: i32 = 0;
const PLAYER_MAX_OVERALL: i32 = 99;
const X_MAX_ATTACK: i32 = PLAYER_MAX_OVERALL * N_ATTACK;
const X_MIN_ATTACK: i32 = PLAYER_MIN_OVERALL * N_ATTACK;
const X_MAX_DEFENSE: i32 = PLAYER_MAX_OVERALL * N_DEFENSE;
const X_MIN_DEFENSE: i32 = PLAYER_MIN_OVERALL * N_DEFENSE;
const X_MAX_MIDFIELD: i32 = PLAYER_MAX_OVERALL * N_MIDFIELD;
const X_MIN_MIDFIELD: i32 = PLAYER_MIN_OVERALL * N_MIDFIELD;

fn generate_poisson_variante(xg: f64) -> i32 {
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

pub fn simulate_match(home_team: Team, away_team: Team, home_squad: Vec<Player>, away_squad: Vec<Player>) -> Matches {
  let mut home_attack: i32 = 0;
  let mut home_defense: i32 = 0;
  let mut home_midfield: i32 = 0;

  let mut away_attack: i32 = 0;
  let mut away_defense: i32 = 0;
  let mut away_midfield: i32 = 0;

  for n in 0..home_squad.len() {
    match home_squad[n].position {
      Position::Attack => home_attack += home_squad[n].overall,
      Position::Defense => home_defense += home_squad[n].overall,
      Position::Middlefier => home_midfield += home_squad[n].overall
    }
  }

  for n in 0..away_squad.len() {
    match away_squad[n].position {
      Position::Attack => away_attack += away_squad[n].overall,
      Position::Defense => away_defense += away_squad[n].overall,
      Position::Middlefier => away_midfield += away_squad[n].overall
    }
  }

  let home_attack_overall = ((home_attack - X_MIN_ATTACK) * 99) / (X_MAX_ATTACK - X_MIN_ATTACK);
  let away_attack_overall = ((away_attack - X_MIN_ATTACK) * 99) / (X_MAX_ATTACK - X_MIN_ATTACK);

  let home_defense_overall = ((home_defense - X_MIN_DEFENSE) * 99) / (X_MAX_DEFENSE - X_MIN_DEFENSE);
  let away_defense_overall = ((away_defense - X_MIN_DEFENSE) * 99) / (X_MAX_DEFENSE - X_MIN_DEFENSE);

  let home_midfield_overall = ((home_midfield - X_MIN_MIDFIELD) * 99) / (X_MAX_MIDFIELD - X_MIN_MIDFIELD);
  let away_midfield_overall = ((away_midfield - X_MIN_MIDFIELD) * 99) / (X_MAX_MIDFIELD - X_MIN_MIDFIELD);

  let home_strength = TeamStatus::new_status(home_attack_overall, home_defense_overall, home_midfield_overall);
  let away_strength = TeamStatus::new_status(away_attack_overall, away_defense_overall, away_midfield_overall);

  let home_xg = 1.5 + (home_strength.attack_overall as f64 - away_strength.defense_overall as f64) * 0.02;
  let away_xg = 1.5 + (away_strength.attack_overall as f64 - home_strength.defense_overall as f64) * 0.02;

  let home_goals = generate_poisson_variante(home_xg);
  let away_goals = generate_poisson_variante(away_xg);

  return Matches { home_uuid: home_team.uuid, away_uuid: away_team.uuid, home_score: home_goals, away_score: away_goals }
}