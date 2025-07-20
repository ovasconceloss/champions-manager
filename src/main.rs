use champions_manager::{core::{matches::model::Matches, player::model::Player, team::model::Team}, engine::simulation::simulate_match};

fn main() {
    let alphabet: Vec<char> = "abcdefghijklmonrstuvwxyz".chars().collect();
    let team_a: Team = Team::new(uuid::Uuid::new_v4(), "Barcelona".to_string(), 100, "FCB".to_string());
    let team_b: Team = Team::new(uuid::Uuid::new_v4(), "Real Madrid".to_string(), 100, "RMA".to_string());

    let mut team_a_squad: Vec<Player> = vec![];
    let mut team_b_squad: Vec<Player> = vec![];

    for n in 0..10 {
        let index: usize = n as usize;
        team_a_squad.push(Player::new(alphabet[index].to_string(), team_a.uuid));
        team_b_squad.push(Player::new(alphabet[index].to_string(), team_b.uuid));
    }

    let result: Matches = simulate_match(team_a.clone(), team_b.clone(), team_a_squad, team_b_squad);
    println!("{:?}", result);
}