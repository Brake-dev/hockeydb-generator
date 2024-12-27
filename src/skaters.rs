use rand::{self, thread_rng, Rng};
use std::ops::RangeInclusive;
use uuid::Uuid;

use crate::birthplaces::get_birthplace_by_nationality;
use crate::season_names::SEASON_NUMS;
use crate::skater_names::get_name_by_nationality;
use crate::team_names::TEAM_NAMES_SHORT;
use crate::utils::format_num_to_ordinal;

#[derive(Debug, Clone)]
pub struct Skater {
    pub skater_id: String,
    pub age: i32,
    pub name: String,
    pub number: i32,
    pub height: String,
    pub weight: i32,
    pub born: String,
    pub birthplace: String,
    pub shoots: String,
    pub draft: String,
    line: i32,
    pub position: String,
}

#[derive(PartialEq)]
pub enum Nationality {
    CAN,
    USA,
    SWE,
    RUS,
    FIN,
    CZE,
}

pub const AGE: RangeInclusive<i32> = 18..=35;
pub const NUMBER: RangeInclusive<i32> = 1..=98;
pub const HEIGHT: [&str; 13] = [
    "5′8″",
    "5′9″",
    "5′10″",
    "5′11″",
    "6′0″",
    "6′1″",
    "6′2″",
    "6′3″",
    "6′4″",
    "6′5″",
    "6′6″",
    "6′7″",
    "6′8″",
];
pub const WEIGHT: RangeInclusive<i32> = 160..=250; // lb
pub const LINE_POSITIONS: [&str; 5] = ["LW", "C", "RW", "D", "D"];

pub fn get_age(season: &i32) -> i32 {
    let first_season = SEASON_NUMS[0];

    let age = if season == &first_season {
        thread_rng().gen_range(AGE)
    } else {
        18
    };

    age
}

pub fn get_birthdate(age: i32, season: &i32) -> String {
    let year = season - age;
    let month = thread_rng().gen_range(1..=12);
    let day = thread_rng().gen_range(1..=29);

    vec![day.to_string(), month.to_string(), year.to_string()].join("/")
}

// 7 rounds, 32 picks/round, 225 picks overall
pub fn get_draft(birth_year: i32) -> String {
    let round_num = thread_rng().gen_range(1..=7);
    let round = format_num_to_ordinal(round_num) + " round";

    let pick_num = thread_rng().gen_range(1..=32);
    let pick = format_num_to_ordinal(pick_num) + " pick";

    let round_adjust = round_num - 1;
    let overall = pick_num + (round_adjust * 32);

    let team_index = thread_rng().gen_range(0..TEAM_NAMES_SHORT.len());
    let draft_team_overall =
        String::from(TEAM_NAMES_SHORT[team_index]) + " (" + &format_num_to_ordinal(overall) + ")";

    let draft_year = birth_year + 18;

    vec![draft_year.to_string(), draft_team_overall, round, pick].join(", ")
}

// CAN 40%, USA 30%, SWE 12%, RUS 8%, FIN 6%, CZE 4%
pub fn get_nationality() -> Nationality {
    let rand = thread_rng().gen_range(1..=100);

    if rand <= 40 {
        return Nationality::CAN;
    } else if rand > 40 && rand <= 70 {
        return Nationality::USA;
    } else if rand > 70 && rand <= 82 {
        return Nationality::SWE;
    } else if rand > 82 && rand <= 90 {
        return Nationality::RUS;
    } else if rand > 90 && rand <= 96 {
        return Nationality::FIN;
    }

    return Nationality::CZE;
}

impl Skater {
    pub fn new(season: &i32, line: i32, pos_index: usize) -> Skater {
        let skater_id = String::from(Uuid::new_v4());
        let age = get_age(season);
        let born = get_birthdate(age, season);
        let draft = get_draft(season - age);
        let nationality = get_nationality();
        let number = thread_rng().gen_range(NUMBER);
        let weight = thread_rng().gen_range(WEIGHT);
        let name = get_name_by_nationality(&nationality);
        let birthplace = get_birthplace_by_nationality(&nationality);

        let height_index = thread_rng().gen_range(0..HEIGHT.len());

        let height = HEIGHT[height_index].to_string();
        let position = LINE_POSITIONS[pos_index].to_string();

        // LW or left D
        let shoots = if pos_index == 0 || pos_index == 3 {
            String::from("L")
        } else {
            String::from("R")
        };

        Skater {
            skater_id,
            age,
            name,
            number,
            height,
            weight,
            born,
            birthplace,
            shoots,
            draft,
            line,
            position,
        }
    }
}
