use std::env;
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

use format::{
    games_to_csv, goalie_stats_to_csv, goalies_season_to_csv, goalies_to_csv, seasons_to_csv,
    skater_stats_to_csv, skaters_season_to_csv, skaters_to_csv, team_games_to_csv,
    team_goalies_to_csv, team_season_to_csv, team_skaters_to_csv, team_stats_to_csv, teams_to_csv,
};
use games::Game;
use generators::{get_entry_draft_data, get_game_data, retire_and_draft_players};
use goalie_stats::GoalieStats;
use goalies::Goalie;
use season_names::SEASON_NAMES;
use seasons::Season;
use skater_stats::SkaterStats;
use skaters::Skater;
use team_stats::TeamStats;
use utils::write_string_to_file;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

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

    let mut team_stats: Vec<TeamStats> = vec![];

    let mut all_games: Vec<Game> = vec![];
    let mut team_games: Vec<(String, String, String)> = vec![];

    let mut all_skaters: Vec<Skater> = vec![];
    let mut all_goalies: Vec<Goalie> = vec![];

    let mut skater_stats: Vec<SkaterStats> = vec![];
    let mut goalie_stats: Vec<GoalieStats> = vec![];

    let mut team_skaters: Vec<(String, String, String)> = vec![];
    let mut team_goalies: Vec<(String, String, String)> = vec![];

    let mut conference_range_end = 48;
    let division_ranges = [5, 4, 1, 0, 0, 1, 4, 5];

    for season in league.seasons {
        let teams_middle = season.teams.len() / 2;

        for (team_i, team) in season.teams.iter().enumerate() {
            let stats = TeamStats::new(&team.team_id, &season.season_id);
            team_stats.push(stats);

            // All inter-conference games
            if team_i < teams_middle {
                let mut inter_i = teams_middle;
                for inter_conference in 1..=32 {
                    let game_res = get_game_data(&inter_conference, inter_i, &season, team);

                    match game_res {
                        Some(res) => {
                            all_games.push(res.0);
                            team_games.push(res.1);
                            team_games.push(res.2);
                        }
                        None => (),
                    }

                    if inter_i + 1 > season.teams.len() - 1 {
                        inter_i = teams_middle;
                    } else {
                        inter_i += 1;
                    }
                }
            }

            let mut conf_i = team_i;

            // Most conference and division games
            for conference in 1..=conference_range_end {
                let game_res = get_game_data(&conference, conf_i, &season, team);

                match game_res {
                    Some(res) => {
                        all_games.push(res.0);
                        team_games.push(res.1);
                        team_games.push(res.2);
                    }
                    None => (),
                }

                if team_i < teams_middle {
                    if conf_i + 1 >= teams_middle {
                        conf_i = team_i;
                    } else {
                        conf_i += 1;
                    }
                } else {
                    if conf_i + 1 >= season.teams.len() {
                        conf_i = team_i;
                    } else {
                        conf_i += 1;
                    }
                }
            }

            if conference_range_end - 3 == 0 {
                conference_range_end = 48;
            } else {
                conference_range_end -= 3;
            }

            // Extra division games
            let div_team_start_i = team_i % 8;
            if div_team_start_i != 3 && div_team_start_i != 4 {
                let division_range = 1..=division_ranges[div_team_start_i];
                let mut div_i = if div_team_start_i < 4 {
                    team_i + 1
                } else {
                    team_i - 1
                };

                for division in division_range {
                    let game_res = get_game_data(&division, div_i, &season, team);

                    match game_res {
                        Some(res) => {
                            all_games.push(res.0);
                            team_games.push(res.1);
                            team_games.push(res.2);
                        }
                        None => (),
                    }

                    if div_team_start_i < 4 {
                        div_i += 1;
                    } else {
                        div_i -= 1;
                    }
                }
            }

            for line in &team.lines {
                for skater in line {
                    team_skaters.push((
                        team.team_id.to_string(),
                        skater.skater_id.to_string(),
                        season.season_id.to_string(),
                    ));

                    let stats = SkaterStats::new(
                        &skater.skater_id,
                        &season.season_id,
                        &skater.line,
                        &skater.position,
                    );
                    skater_stats.push(stats);

                    if !all_skaters.iter().any(|s| s.skater_id == skater.skater_id) {
                        all_skaters.push(skater.clone());
                    }
                }
            }

            for goalie in &team.goalies {
                team_goalies.push((
                    team.team_id.to_string(),
                    goalie.goalie_id.to_string(),
                    season.season_id.to_string(),
                ));

                let stats = GoalieStats::new(&goalie.goalie_id, &season.season_id, &goalie.index);
                goalie_stats.push(stats);

                if !all_goalies.iter().any(|g| g.goalie_id == goalie.goalie_id) {
                    all_goalies.push(goalie.clone());
                }
            }
        }
    }

    let team_stats_formatted = team_stats_to_csv(team_stats);
    let games_formatted = games_to_csv(all_games);
    let team_games_formatted = team_games_to_csv(team_games);

    let skaters_formatted = skaters_to_csv(all_skaters);
    let goalies_formatted = goalies_to_csv(all_goalies);

    let team_skaters_formatted = team_skaters_to_csv(&team_skaters);
    let skater_season_formatted = skaters_season_to_csv(&team_skaters);

    let skater_stats_formatted = skater_stats_to_csv(skater_stats);

    let team_goalies_formatted = team_goalies_to_csv(&team_goalies);
    let goalie_season_formatted = goalies_season_to_csv(&team_goalies);

    let goalie_stats_formatted = goalie_stats_to_csv(goalie_stats);

    let _ = write_string_to_file("./output/seasons.csv", seasons_formatted);

    let _ = write_string_to_file("./output/teams.csv", teams_formatted);
    let _ = write_string_to_file("./output/team_seasons.csv", team_seasons_formatted);
    let _ = write_string_to_file("./output/team_stats.csv", team_stats_formatted);

    let _ = write_string_to_file("./output/games.csv", games_formatted);
    let _ = write_string_to_file("./output/team_games.csv", team_games_formatted);

    let _ = write_string_to_file("./output/skaters.csv", skaters_formatted);
    let _ = write_string_to_file("./output/team_skaters.csv", team_skaters_formatted);
    let _ = write_string_to_file("./output/skater_stats.csv", skater_stats_formatted);

    let _ = write_string_to_file("./output/goalies.csv", goalies_formatted);
    let _ = write_string_to_file("./output/team_goalies.csv", team_goalies_formatted);
    let _ = write_string_to_file("./output/goalie_stats.csv", goalie_stats_formatted);

    let _ = write_string_to_file("./output/skaters_seasons.csv", skater_season_formatted);
    let _ = write_string_to_file("./output/goalie_seasons.csv", goalie_season_formatted);

    println!("Finished in {}ms", timer.elapsed().as_millis());
}
