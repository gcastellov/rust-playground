use std::collections::HashMap;

const HEADER: &str = "Team                           | MP |  W |  D |  L |  P";
const NEW_LINE: char = '\n';
const SEPARATOR: char = ';';

struct Stats {
    team: String,
    matches_played: u8,
    matches_won: u8,
    matches_drawn: u8,
    matches_lost: u8,
    points: u8,
}

impl Stats {
    fn default(team: String) -> Self {
        Stats {
            team,
            matches_played: 0,
            matches_won: 0,
            matches_drawn: 0,
            matches_lost: 0,
            points: 0,
        }
    }

    fn victory(team_a: &str) -> Self {
        Stats {
            team: String::from(team_a),
            matches_played: 1,
            matches_won: 1,
            matches_drawn: 0,
            matches_lost: 0,
            points: 3,
        }
    }

    fn defeat(team: &str) -> Self {
        Stats {
            team: String::from(team),
            matches_played: 1,
            matches_won: 0,
            matches_drawn: 0,
            matches_lost: 1,
            points: 0,
        }
    }

    fn tie(team: &str) -> Self {
        Stats {
            team: String::from(team),
            matches_played: 1,
            matches_won: 0,
            matches_drawn: 1,
            matches_lost: 0,
            points: 1,
        }
    }

    fn add(&self, other: &Stats) -> Self {
        if self.team != other.team {
            panic!("Invalid stats provided");
        }

        Stats {
            team: self.team.clone(),
            matches_played: self.matches_played + other.matches_played,
            matches_won: self.matches_won + other.matches_won,
            matches_drawn: self.matches_drawn + other.matches_drawn,
            matches_lost: self.matches_lost + other.matches_lost,
            points: self.points + other.points,
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut table: String = String::from(HEADER);
    let mut stats_by_team: HashMap<String, Stats> = HashMap::default();
    match_results
        .split(NEW_LINE)
        .filter(|entry| !entry.is_empty())
        .map(|entry| {
            let stats: Vec<&str> = entry.split(SEPARATOR).collect();
            let mut mach_stats: Vec<Stats> = Vec::with_capacity(2);
            match *stats.last().unwrap() {
                "win" => {
                    mach_stats.push(Stats::victory(stats[0]));
                    mach_stats.push(Stats::defeat(stats[1]));
                }
                "loss" => {
                    mach_stats.push(Stats::defeat(stats[0]));
                    mach_stats.push(Stats::victory(stats[1]));
                }
                _ => {
                    mach_stats.push(Stats::tie(stats[0]));
                    mach_stats.push(Stats::tie(stats[1]));
                }
            };
            mach_stats
        })
        .flatten()
        .for_each(|stats| {
            let current_stats: Stats;
            if let Some(current) = stats_by_team.get(&stats.team.clone()) {
                current_stats = current.add(&stats);
            } else {
                current_stats = Stats::default(stats.team.clone()).add(&stats);
            }

            stats_by_team.insert(stats.team, current_stats);
        });

    let mut sorted_stats: Vec<&Stats> = stats_by_team.iter().map(|(_, s)| s).collect();
    sorted_stats.sort_by(|a, b| {
        b.points
            .cmp(&a.points)
            .then(a.team.partial_cmp(&b.team).unwrap())
    });

    for team_stats in sorted_stats {
        let line = format!("\n{: <31}|{: >3} |{: >3} |{: >3} |{: >3} |{: >3}",
            team_stats.team,
            team_stats.matches_played,
            team_stats.matches_won,
            team_stats.matches_drawn,
            team_stats.matches_lost,
            team_stats.points);
        table.push_str(line.as_str());
    }

    table
}
