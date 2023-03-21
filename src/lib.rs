use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;

mod tests;

struct TeamRecord {
    wins: usize,
    losses: usize,
    draws: usize,
}

impl TeamRecord {
    fn points(&self) -> usize {
        self.wins * 3 + self.draws
    }
}

fn record_games(match_results: &str) -> HashMap<&str, TeamRecord> {
    let mut teams = HashMap::new();
    for game in match_results.split("\n").filter(|game| !game.is_empty()) {
        let (home_team, visiting_team, outcome) = game.split(";").collect_tuple().unwrap();
        match outcome {
            "win" => {
                teams
                    .entry(home_team)
                    .and_modify(|record: &mut TeamRecord| record.wins += 1)
                    .or_insert(TeamRecord {
                        wins: 1,
                        losses: 0,
                        draws: 0,
                    });
                teams
                    .entry(visiting_team)
                    .and_modify(|record: &mut TeamRecord| record.losses += 1)
                    .or_insert(TeamRecord {
                        wins: 0,
                        losses: 1,
                        draws: 0,
                    });
            }
            "loss" => {
                teams
                    .entry(home_team)
                    .and_modify(|record: &mut TeamRecord| record.losses += 1)
                    .or_insert(TeamRecord {
                        wins: 0,
                        losses: 1,
                        draws: 0,
                    });
                teams
                    .entry(visiting_team)
                    .and_modify(|record: &mut TeamRecord| record.wins += 1)
                    .or_insert(TeamRecord {
                        wins: 1,
                        losses: 0,
                        draws: 0,
                    });
            }
            "draw" => {
                teams
                    .entry(home_team)
                    .and_modify(|record: &mut TeamRecord| record.draws += 1)
                    .or_insert(TeamRecord {
                        wins: 0,
                        losses: 0,
                        draws: 1,
                    });
                teams
                    .entry(visiting_team)
                    .and_modify(|record: &mut TeamRecord| record.draws += 1)
                    .or_insert(TeamRecord {
                        wins: 0,
                        losses: 0,
                        draws: 1,
                    });
            }
            _ => panic!(),
        }
    }
    teams
}

fn tally_table(teams: &HashMap<&str, TeamRecord>) -> String {
    // let maximum_name_length: usize = teams.keys().map(|name| name.len()).max().unwrap();
    let mut table = format!("{: <31}", "Team") + "| MP |  W |  D |  L |  P";
    for (team_name, team_record) in
        teams
            .iter()
            .sorted_by(|(name_1, record_1), (name_2, record_2)| {
                let points_comparison = record_2.points().cmp(&record_1.points());
                match points_comparison {
                    Ordering::Equal => name_1.cmp(name_2),
                    _ => points_comparison,
                }
            })
    {
        table += format!(
            "\n{:30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
            team_name,
            team_record.wins + team_record.draws + team_record.losses,
            team_record.wins,
            team_record.draws,
            team_record.losses,
            team_record.points()
        )
        .as_str();
    }
    table
}

pub fn tally(match_results: &str) -> String {
    let teams = record_games(match_results);
    tally_table(&teams)
}
