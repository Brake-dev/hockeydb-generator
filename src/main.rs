use std::error::Error;
use std::fs;

mod birthplaces;
mod games;
mod goalie_stats;
mod goalies;
mod league;
mod season_names;
mod seasons;
mod skater_names;
mod skater_stats;
mod skaters;
mod team_names;
mod team_stats;
mod teams;
mod utils;

use season_names::SEASON_NAMES;

fn main() {
    let mut league = league::League::new();

    for (i, season) in SEASON_NAMES.iter().enumerate() {
        // if i != 0 {
        //     break;
        // }

        let mut season = seasons::Season::new(season.to_string(), i);

        for (team_index, team) in team_names::TEAM_NAMES.iter().enumerate() {
            let mut team = teams::Team::new(team.to_string(), team_index);

            for (line_index, line) in (1..=4).into_iter().enumerate() {
                for (pos_index, _) in (1..=5).into_iter().enumerate() {
                    let skater = skaters::Skater::new(&season.season_num, line, pos_index);
                    team.lines[line_index].push(skater);
                }
            }

            for _ in 1..=2 {
                let goalie = goalies::Goalie::new(&season.season_num);
                team.goalies.push(goalie);
            }

            season.teams.push(team);
        }

        league.seasons.push(season);
    }

    // println!("{}", league.seasons.len());
    // println!("{:?}", league);

    // let _ = write_string_to_file("results.txt", "hello");
}

fn write_string_to_file(path: &str, data: &str) -> Result<(), Box<dyn Error>> {
    fs::write(path, data)?;
    Ok(())
}
