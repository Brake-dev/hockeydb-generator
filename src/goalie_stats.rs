use rand::{self, thread_rng, Rng};
use std::ops::RangeInclusive;

use crate::utils::{
    adjust_float_range_by_goalie_index, adjust_range_by_line_and_pos,
    adjust_range_by_line_and_pos_low_is_better,
};

pub const GAMES_PLAYED: RangeInclusive<i32> = 20..=64;
pub const GAMES_STARTED: RangeInclusive<i32> = 20..=64;
pub const WINS: RangeInclusive<i32> = 0..=38;
pub const LOSSES: RangeInclusive<i32> = 0..=31;
pub const OVERTIME_LOSSES: RangeInclusive<i32> = 0..=10;
pub const SHOTS_AGAINST: RangeInclusive<i32> = 500..=1845;
pub const GOALS_AGAINST_AVERAGE: RangeInclusive<f32> = 2.00..=5.00;
pub const SAVE_PERCENTAGE: RangeInclusive<f32> = 0.870..=0.920;
pub const SHUTOUT: RangeInclusive<i32> = 0..=6;
pub const GOALS: RangeInclusive<i32> = 0..=1;
pub const ASSISTS: RangeInclusive<i32> = 0..=5;
pub const PENALTY_MINUTES: RangeInclusive<i32> = 0..=14;
pub const TIME_ON_ICE: RangeInclusive<i32> = 980..=3630;

pub struct GoalieStats {
    pub goalie_id: String,
    pub season_id: String,
    pub games_played: i32,
    pub games_started: i32,
    pub wins: i32,
    pub losses: i32,
    pub overtime_losses: i32,
    pub shots_against: i32,
    pub goals_against_average: f32,
    pub save_percentage: f32,
    pub shutout: i32,
    pub goals: i32,
    pub assists: i32,
    pub penalty_minutes: i32,
    pub time_on_ice: i32,
}

impl GoalieStats {
    pub fn new(goalie_id: &String, season_id: &String, goalie_index: &usize) -> GoalieStats {
        let adjust_index_to_line = if *goalie_index == 0 { 1 } else { 3 };
        let position = String::from("G");

        let games_played = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &GAMES_PLAYED,
            &adjust_index_to_line,
            &position,
        ));

        let games_started = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &(*GAMES_STARTED.start()..=games_played),
            &adjust_index_to_line,
            &position,
        ));

        let wins = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &WINS,
            &adjust_index_to_line,
            &position,
        ));

        let losses = thread_rng().gen_range(adjust_range_by_line_and_pos_low_is_better(
            &LOSSES,
            &adjust_index_to_line,
            &position,
        ));

        let overtime_losses = thread_rng().gen_range(adjust_range_by_line_and_pos_low_is_better(
            &OVERTIME_LOSSES,
            &adjust_index_to_line,
            &position,
        ));

        let shots_against = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &SHOTS_AGAINST,
            &adjust_index_to_line,
            &position,
        ));

        let goals_against_average = thread_rng().gen_range(adjust_float_range_by_goalie_index(
            &GOALS_AGAINST_AVERAGE,
            &goalie_index,
            true,
        ));

        let save_percentage = thread_rng().gen_range(adjust_float_range_by_goalie_index(
            &SAVE_PERCENTAGE,
            &goalie_index,
            false,
        ));

        let shutout = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &SHUTOUT,
            &adjust_index_to_line,
            &position,
        ));

        let goals = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &GOALS,
            &adjust_index_to_line,
            &position,
        ));

        let assists = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &ASSISTS,
            &adjust_index_to_line,
            &position,
        ));

        let penalty_minutes = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &PENALTY_MINUTES,
            &adjust_index_to_line,
            &position,
        ));

        let time_on_ice = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &TIME_ON_ICE,
            &adjust_index_to_line,
            &position,
        ));

        GoalieStats {
            goalie_id: goalie_id.to_string(),
            season_id: season_id.to_string(),
            games_played,
            games_started,
            wins,
            losses,
            overtime_losses,
            shots_against,
            goals_against_average,
            save_percentage,
            shutout,
            goals,
            assists,
            penalty_minutes,
            time_on_ice,
        }
    }
}
