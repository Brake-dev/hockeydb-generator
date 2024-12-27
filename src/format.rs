use crate::goalies::Goalie;
use crate::seasons::Season;
use crate::skaters::Skater;
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

pub fn skaters_to_csv(skaters: &Vec<Skater>) -> String {
    let mut rows =
        String::from("skater_id;name;number;height;weight;born;birthplace;shoots;draft;position\n");

    for skater in skaters {
        rows.push_str(
            &(vec![
                skater.skater_id.to_string(),
                skater.name.to_string(),
                skater.number.to_string(),
                skater.height.to_string(),
                skater.weight.to_string(),
                skater.born.to_string(),
                skater.birthplace.to_string(),
                skater.shoots.to_string(),
                skater.draft.to_string(),
                skater.position.to_string(),
            ]
            .join(";")
                + "\n"),
        );
    }

    rows
}

pub fn goalies_to_csv(goalies: &Vec<Goalie>) -> String {
    let mut rows =
        String::from("goalie_id;name;number;height;weight;born;birthplace;catches;draft\n");

    for goalie in goalies {
        rows.push_str(
            &(vec![
                goalie.goalie_id.to_string(),
                goalie.name.to_string(),
                goalie.number.to_string(),
                goalie.height.to_string(),
                goalie.weight.to_string(),
                goalie.born.to_string(),
                goalie.birthplace.to_string(),
                goalie.catches.to_string(),
                goalie.draft.to_string(),
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
