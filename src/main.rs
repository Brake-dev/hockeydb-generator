use std::error::Error;
use std::fs;
use std::time;

mod birthplaces;
mod format;
mod games;
mod generators;
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

use format::{goalies_to_csv, seasons_to_csv, skaters_to_csv, team_season_to_csv, teams_to_csv};
use generators::{get_entry_draft_data, retire_and_draft_players};
use goalies::Goalie;
use season_names::SEASON_NAMES;
use seasons::Season;
use skaters::Skater;

fn main() {
    let timer = time::Instant::now();
    let mut league = league::League::new();

    for (season_index, season) in SEASON_NAMES.iter().enumerate() {
        let mut season = Season::new(season.to_string(), season_index);

        if season_index == 0 {
            season = get_entry_draft_data(season);
        } else {
            season = retire_and_draft_players(&league, season_index, season);
        }

        league.seasons.push(season);
    }

    let seasons_formatted = seasons_to_csv(&league.seasons);
    let teams_formatted = teams_to_csv(&league.seasons[0].teams);
    let team_seasons_formatted = team_season_to_csv(&league.seasons, &league.seasons[0].teams);

    let mut all_skaters: Vec<Skater> = vec![];
    let mut all_goalies: Vec<Goalie> = vec![];

    for season in league.seasons {
        for team in season.teams {
            for line in team.lines {
                for skater in line {
                    if !all_skaters.iter().any(|s| s.skater_id == skater.skater_id) {
                        all_skaters.push(skater);
                    }
                }
            }

            for goalie in team.goalies {
                if !all_goalies.iter().any(|g| g.goalie_id == goalie.goalie_id) {
                    all_goalies.push(goalie);
                }
            }
        }
    }

    let skaters_formatted = skaters_to_csv(&all_skaters);
    let goalies_formatted = goalies_to_csv(&all_goalies);

    let _ = write_string_to_file("./output/seasons.csv", seasons_formatted);
    let _ = write_string_to_file("./output/teams.csv", teams_formatted);
    let _ = write_string_to_file("./output/team_seasons.csv", team_seasons_formatted);
    let _ = write_string_to_file("./output/skaters.csv", skaters_formatted);
    let _ = write_string_to_file("./output/goalies.csv", goalies_formatted);

    println!("Finished in {}ms", timer.elapsed().as_millis());
}

fn write_string_to_file(path: &str, data: String) -> Result<(), Box<dyn Error>> {
    fs::write(path, data)?;
    Ok(())
}
