use crate::games::Game;
use crate::goalie_stats::GoalieStats;
use crate::goalies::Goalie;
use crate::seasons::Season;
use crate::skater_stats::SkaterStats;
use crate::skaters::Skater;
use crate::team_stats::TeamStats;
use crate::teams::Team;

pub fn seasons_to_csv(seasons: &Vec<Season>) -> String {
    let mut rows = String::from("season_id;name\n");

    for season in seasons {
        rows.push_str(
            &(vec![season.season_id.to_string(), season.season_name.to_string()].join(";") + "\n"),
        );
    }

    rows
}

pub fn teams_to_csv(teams: &Vec<Team>) -> String {
    let mut rows = String::from("team_id;name;conference;division\n");

    for team in teams {
        rows.push_str(
            &(vec![
                team.team_id.to_string(),
                team.team_name.to_string(),
                team.conference.to_string(),
                team.division.to_string(),
            ]
            .join(";")
                + "\n"),
        );
    }

    rows
}

pub fn team_season_to_csv(seasons: &Vec<Season>, teams: &Vec<Team>) -> String {
    let mut rows = String::from("team_id;season_id\n");

    for season in seasons {
        for team in teams {
            rows.push_str(
                &(vec![team.team_id.to_string(), season.season_id.to_string()].join(";") + "\n"),
            );
        }
    }

    rows
}

pub fn skaters_to_csv(skaters: Vec<Skater>) -> String {
    let mut rows =
        String::from("skater_id;name;number;height;weight;born;birthplace;shoots;draft;position\n");

    for skater in skaters {
        rows.push_str(
            &(vec![
                skater.skater_id,
                skater.name,
                skater.number.to_string(),
                skater.height,
                skater.weight.to_string(),
                skater.born,
                skater.birthplace,
                skater.shoots,
                skater.draft,
                skater.position,
            ]
            .join(";")
                + "\n"),
        );
    }

    rows
}

pub fn goalies_to_csv(goalies: Vec<Goalie>) -> String {
    let mut rows =
        String::from("goalie_id;name;number;height;weight;born;birthplace;catches;draft\n");

    for goalie in goalies {
        rows.push_str(
            &(vec![
                goalie.goalie_id,
                goalie.name,
                goalie.number.to_string(),
                goalie.height,
                goalie.weight.to_string(),
                goalie.born,
                goalie.birthplace,
                goalie.catches,
                goalie.draft,
            ]
            .join(";")
                + "\n"),
        );
    }

    rows
}

pub fn team_skaters_to_csv(ids: &Vec<(String, String, String)>) -> String {
    let mut rows = String::from("team_id;skater_id;season_id\n");

    for id in ids {
        rows.push_str(
            &(vec![id.0.to_string(), id.1.to_string(), id.2.to_string()].join(";") + "\n"),
        );
    }

    rows
}

pub fn skaters_season_to_csv(ids: &Vec<(String, String, String)>) -> String {
    let mut rows = String::from("skater_id;season_id\n");

    for id in ids {
        rows.push_str(&(vec![id.1.to_string(), id.2.to_string()].join(";") + "\n"));
    }

    rows
}

pub fn team_goalies_to_csv(ids: &Vec<(String, String, String)>) -> String {
    let mut rows = String::from("team_id;goalie_id;season_id\n");

    for id in ids {
        rows.push_str(
            &(vec![id.0.to_string(), id.1.to_string(), id.2.to_string()].join(";") + "\n"),
        );
    }

    rows
}

pub fn goalies_season_to_csv(ids: &Vec<(String, String, String)>) -> String {
    let mut rows = String::from("goalie_id;season_id\n");

    for id in ids {
        rows.push_str(&(vec![id.1.to_string(), id.2.to_string()].join(";") + "\n"));
    }

    rows
}

pub fn team_stats_to_csv(stats: Vec<TeamStats>) -> String {
    let mut rows = String::from("team_id;season_id;games;wins;losses;overtime_wins;overtime_losses;points;goals_for;goals_against;home_record;away_record;shootout_record;last_10;streak\n");

    for stat in stats {
        rows.push_str(
            &(vec![
                stat.team_id,
                stat.season_id,
                stat.games.to_string(),
                stat.wins.to_string(),
                stat.losses.to_string(),
                stat.overtime_wins.to_string(),
                stat.overtime_losses.to_string(),
                stat.points.to_string(),
                stat.goals_for.to_string(),
                stat.goals_against.to_string(),
                stat.home_record,
                stat.away_record,
                stat.shootout_record,
                stat.last_10,
                stat.streak,
            ]
            .join(";")
                + "\n"),
        );
    }

    rows
}

pub fn skater_stats_to_csv(stats: Vec<SkaterStats>) -> String {
    let mut rows = String::from("skater_id;season_id;games_played;goals;assists;points;plus_minus;penalty_minutes;powerplay_goals;powerplay_points;shorthanded_goals;shorthanded_points;time_on_ice_per_gp;game_winning_goals;overtime_goals;shots_on_goal;face_off_percentage\n");

    for stat in stats {
        rows.push_str(
            &(vec![
                stat.skater_id,
                stat.season_id,
                stat.games_played.to_string(),
                stat.goals.to_string(),
                stat.assists.to_string(),
                stat.points.to_string(),
                stat.plus_minus.to_string(),
                stat.penalty_minutes.to_string(),
                stat.powerplay_goals.to_string(),
                stat.powerplay_points.to_string(),
                stat.shorthanded_goals.to_string(),
                stat.shorthanded_points.to_string(),
                stat.time_on_ice_per_gp,
                stat.game_winning_goals.to_string(),
                stat.overtime_goals.to_string(),
                stat.shots_on_goal.to_string(),
                stat.face_off_percentage.to_string(),
            ]
            .join(";")
                + "\n"),
        );
    }

    rows
}

pub fn goalie_stats_to_csv(stats: Vec<GoalieStats>) -> String {
    let mut rows = String::from("goalie_id;season_id;games_played;games_started;wins;losses;overtime_losses;shots_against;goals_against_average;save_percentage;shutouts;goals;assists;penalty_minutes;time_on_ice\n");

    for stat in stats {
        rows.push_str(
            &(vec![
                stat.goalie_id,
                stat.season_id,
                stat.games_played.to_string(),
                stat.games_started.to_string(),
                stat.wins.to_string(),
                stat.losses.to_string(),
                stat.overtime_losses.to_string(),
                stat.shots_against.to_string(),
                stat.goals_against_average.to_string(),
                stat.save_percentage.to_string(),
                stat.shutout.to_string(),
                stat.goals.to_string(),
                stat.assists.to_string(),
                stat.penalty_minutes.to_string(),
                stat.time_on_ice.to_string(),
            ]
            .join(";")
                + "\n"),
        );
    }

    rows
}

pub fn games_to_csv(games: Vec<Game>) -> String {
    let mut rows = String::from("game_id;season_id;home_team;away_team;overtime;shootout;");
    rows.push_str("home_team_goals;home_team_shots_on_goal;home_team_face_off_percent;home_team_powerplay_percent;home_team_penalty_minutes;home_team_hits;home_team_blocked_shots;home_team_giveaways;home_team_takeaways;");
    rows.push_str("away_team_goals;away_team_shots_on_goal;away_team_face_off_percent;away_team_powerplay_percent;away_team_penalty_minutes;away_team_hits;away_team_blocked_shots;away_team_giveaways;away_team_takeaways\n");

    for game in games {
        rows.push_str(
            &(vec![
                game.game_id,
                game.season_id,
                game.home_team_id,
                game.away_team_id,
                game.overtime.to_string(),
                game.shootout.to_string(),
                game.home_goals.to_string(),
                game.home_shots_on_goal.to_string(),
                game.home_face_off_percentage.to_string(),
                game.home_powerplay_percentage.to_string(),
                game.home_penalty_minutes.to_string(),
                game.home_hits.to_string(),
                game.home_blocked_shots.to_string(),
                game.home_giveaways.to_string(),
                game.home_takeaways.to_string(),
                game.away_goals.to_string(),
                game.away_shots_on_goal.to_string(),
                game.away_face_off_percentage.to_string(),
                game.away_powerplay_percentage.to_string(),
                game.away_penalty_minutes.to_string(),
                game.away_hits.to_string(),
                game.away_blocked_shots.to_string(),
                game.away_giveaways.to_string(),
                game.away_takeaways.to_string(),
            ]
            .join(";")
                + "\n"),
        );
    }

    rows
}

pub fn team_games_to_csv(ids: Vec<(String, String, String)>) -> String {
    let mut rows = String::from("game_id;team_id;season_id\n");

    for id in ids {
        rows.push_str(
            &(vec![id.0.to_string(), id.1.to_string(), id.2.to_string()].join(";") + "\n"),
        );
    }

    rows
}
