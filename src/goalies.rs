use rand::{self, thread_rng, Rng};
use uuid::Uuid;

use crate::birthplaces::get_birthplace_by_nationality;
use crate::skater_names::get_name_by_nationality;
use crate::skaters::{get_age, get_birthdate, get_draft, get_nationality, HEIGHT, NUMBER, WEIGHT};

pub const CATCHES: [&str; 2] = ["L", "R"];

#[derive(Debug, Clone)]
pub struct Goalie {
    pub goalie_id: String,
    pub age: i32,
    pub name: String,
    pub number: i32,
    pub height: String,
    pub weight: i32,
    pub born: String,
    pub birthplace: String,
    pub catches: String,
    pub draft: String,
}

impl Goalie {
    pub fn new(season: &i32) -> Goalie {
        let goalie_id = String::from(Uuid::new_v4());
        let age = get_age(season);
        let born = get_birthdate(age, season);
        let draft = get_draft(season - age);
        let nationality = get_nationality();
        let number = thread_rng().gen_range(NUMBER);
        let weight = thread_rng().gen_range(WEIGHT);
        let name = get_name_by_nationality(&nationality);
        let birthplace = get_birthplace_by_nationality(&nationality);

        let height_index = thread_rng().gen_range(0..HEIGHT.len());
        let catches_index = thread_rng().gen_range(0..CATCHES.len());

        let height = HEIGHT[height_index].to_string();
        let catches = CATCHES[catches_index].to_string();

        Goalie {
            goalie_id,
            age,
            name,
            number,
            height,
            weight,
            born,
            birthplace,
            catches,
            draft,
        }
    }
}
