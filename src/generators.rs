use crate::goalies::Goalie;
use crate::league::League;
use crate::seasons::Season;
use crate::skaters::Skater;
use crate::team_names::TEAM_NAMES;
use crate::teams::Team;

pub fn get_entry_draft_data(mut season: Season) -> Season {
    for (team_index, team) in TEAM_NAMES.iter().enumerate() {
        let mut team = Team::new(team.to_string(), team_index);

        for (line_index, line) in (1..=4).into_iter().enumerate() {
            for (pos_index, _) in (1..=5).into_iter().enumerate() {
                let skater = Skater::new(&season.season_num, line, pos_index);
                team.lines[line_index].push(skater);
            }
        }

        for _ in 1..=2 {
            let goalie = Goalie::new(&season.season_num);
            team.goalies.push(goalie);
        }

        season.teams.push(team);
    }

    season
}

pub fn retire_and_draft_players(
    league: &League,
    season_index: usize,
    mut season: Season,
) -> Season {
    let prev_season_res = league.get_prev_season(season_index);
    match prev_season_res {
        Some(prev_season) => {
            for prev_team in prev_season.teams {
                let mut new_team = prev_team.clone();
                let mut new_lines = new_team.lines;
                for (line_i, line) in prev_team.lines.iter().enumerate() {
                    for (skater_i, skater) in line.iter().enumerate() {
                        if skater.age + 1 > 35 {
                            let new_skater =
                                Skater::new(&season.season_num, (line_i + 1) as i32, skater_i);

                            new_lines[line_i][skater_i] = new_skater;
                        } else {
                            new_lines[line_i][skater_i].age += 1;
                        }
                    }
                }
                new_team.lines = new_lines;

                let mut new_goalies = new_team.goalies;
                for (goalie_i, goalie) in prev_team.goalies.iter().enumerate() {
                    if goalie.age + 1 > 35 {
                        let new_goalie = Goalie::new(&season.season_num);
                        new_goalies[goalie_i] = new_goalie;
                    } else {
                        new_goalies[goalie_i].age += 1;
                    }
                }
                new_team.goalies = new_goalies;

                season.teams.push(new_team);
            }
        }
        None => println!("Problem getting previous season"),
    }

    season
}
