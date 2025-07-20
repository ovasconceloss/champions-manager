use crate::{core::{matches::model::Matches, player::model::Player, team::model::{Team, TeamStatus}}, engine::utils::{calculate_team_strength, generate_poisson_variante, normalize_overall}};

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

pub fn simulate_match(home_team: Team, away_team: Team, home_squad: &[Player], away_squad: &[Player]) -> Matches {
  let (home_attack, home_defense, home_midfield) = calculate_team_strength(&home_squad);
  let (away_attack, away_defense, away_midfield) = calculate_team_strength(&away_squad);

  let home_attack_overall = normalize_overall(home_attack, X_MIN_ATTACK, X_MAX_ATTACK);
  let home_defense_overall = normalize_overall(home_defense, X_MIN_DEFENSE, X_MAX_DEFENSE);
  let home_midfield_overall = normalize_overall(home_midfield, X_MIN_MIDFIELD, X_MAX_MIDFIELD);

  let away_attack_overall = normalize_overall(away_attack, X_MIN_ATTACK, X_MAX_ATTACK);
  let away_defense_overall = normalize_overall(away_defense, X_MIN_DEFENSE, X_MAX_DEFENSE);
  let away_midfield_overall = normalize_overall(away_midfield, X_MIN_MIDFIELD, X_MAX_MIDFIELD);

  let home_strength = TeamStatus::new_status(home_attack_overall, home_defense_overall, home_midfield_overall);
  let away_strength = TeamStatus::new_status(away_attack_overall, away_defense_overall, away_midfield_overall);

  let home_xg = 1.5 + (home_strength.attack_overall as f64 - away_strength.defense_overall as f64) * 0.02;
  let away_xg = 1.5 + (away_strength.attack_overall as f64 - home_strength.defense_overall as f64) * 0.02;

  let home_goals = generate_poisson_variante(home_xg);
  let away_goals = generate_poisson_variante(away_xg);

  return Matches { home_uuid: home_team.uuid, away_uuid: away_team.uuid, home_score: home_goals, away_score: away_goals }
}