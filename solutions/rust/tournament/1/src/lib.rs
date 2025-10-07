
use itertools::Itertools; // 0.8.0
use std::{cmp::Ordering, collections::HashMap};

struct Team<'a>{
    name: &'a str,
    wins: u32,
    ties: u32,
    losses: u32
}

fn calculate_points(team: &Team) -> u32 {
    return team.wins * 3 + team.ties;
}

pub fn tally(match_results: &str) -> String {
    let mut teams: HashMap<&str, Team> = HashMap::new();
    for result in match_results.lines() {
        let tokens: Vec<&str> = result.split(";").collect();
        if tokens.len() != 3 {
            panic!("Invalid input")
        }

        match tokens[2] {
            "win" => {
                teams.entry(tokens[0]).and_modify(|e| e.wins += 1).or_insert(Team { name: tokens[0], wins: 1, ties: 0, losses: 0 });
                teams.entry(tokens[1]).and_modify(|e| e.losses +=1 ).or_insert(Team { name: tokens[1], wins: 0, ties: 0, losses: 1 });
            }
            "loss" => {
                
                    teams.entry(tokens[1]).and_modify(|e| e.wins += 1).or_insert(Team { name: tokens[1], wins: 1, ties: 0, losses: 0 });
                    teams.entry(tokens[0]).and_modify(|e| e.losses +=1 ).or_insert(Team { name: tokens[0], wins: 0, ties: 0, losses: 1 });
                
            }
            "draw" => {
                {
                    teams.entry(tokens[0]).and_modify(|e| e.ties += 1).or_insert(Team { name: tokens[0], wins: 0, ties: 1, losses: 0 });
                    teams.entry(tokens[1]).and_modify(|e| e.ties +=1 ).or_insert(Team { name: tokens[1], wins: 0, ties: 1, losses: 0 });
                }
            }
            _ => panic!("Invalid input")
        }
    }

    let mut output_lines = vec![];
    output_lines.push(format!("{:<30} | MP |  W |  D |  L |  P", "Team"));
    for team in teams.values().sorted_by(|&a, &b| {
        let comparison = Ord::cmp(&calculate_points(b), &calculate_points(a));
        if comparison == Ordering::Equal {
            Ord::cmp(a.name, b.name)
        } else {
            comparison
        }
    }) {
        output_lines.push(format!("{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}", team.name, team.wins + team.ties + team.losses, team.wins, team.ties, team.losses, calculate_points(team)));
    }
    output_lines.join("\n")
}
