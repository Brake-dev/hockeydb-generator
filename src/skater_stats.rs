use rand::{self, thread_rng, Rng};
use std::ops::RangeInclusive;
use std::time::Duration;

use crate::utils::{adjust_range_by_line_and_pos, adjust_range_by_line_and_pos_allow_negative};

pub const GAMES_PLAYED: RangeInclusive<i32> = 78..=82;
pub const GOALS: RangeInclusive<i32> = 0..=70;
pub const ASSISTS: RangeInclusive<i32> = 0..=100;
pub const PLUS_MINUS: RangeInclusive<i32> = -60..=60;
pub const PENALTY_MINUTES: RangeInclusive<i32> = 150..=0;
pub const TIME_ON_ICE_PER_GP: RangeInclusive<i32> = 120..=1500; // In seconds, 2:00 - 25:00
pub const SHOTS_ON_GOAL: RangeInclusive<i32> = 100..=400;
pub const FACE_OFF_PERCENTAGE: RangeInclusive<i32> = 0..=60;

pub struct SkaterStats {
    pub games_played: i32,
    pub goals: i32,
    pub assists: i32,
    pub points: i32,
    pub plus_minus: i32,
    pub penalty_minutes: i32,
    pub powerplay_goals: i32,
    pub powerplay_points: i32,
    pub shorthanded_goals: i32,
    pub shorthanded_points: i32,
    pub overtime_goals: i32,
    pub time_on_ice_per_gp: Duration,
    pub game_winning_goals: i32,
    pub shots_on_goal: i32,
    pub face_off_percentage: i32,
}

impl SkaterStats {
    pub fn new(line: i32, position: String) -> SkaterStats {
        let goals = thread_rng().gen_range(adjust_range_by_line_and_pos(&GOALS, &line, &position));
        let assists =
            thread_rng().gen_range(adjust_range_by_line_and_pos(&ASSISTS, &line, &position));

        let points = goals + assists;

        let plus_minus = thread_rng().gen_range(adjust_range_by_line_and_pos_allow_negative(
            &PLUS_MINUS,
            &line,
            &position,
        ));

        let penalty_minutes = thread_rng().gen_range(adjust_range_by_line_and_pos_allow_negative(
            &PENALTY_MINUTES,
            &line,
            &position,
        ));

        let powerplay_goals = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &(0..=(goals as f64 / 2.00) as i32),
            &line,
            &position,
        ));

        let powerplay_points = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &(0..=(points as f64 / 2.00) as i32),
            &line,
            &position,
        ));

        let shorthanded_goals = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &(0..=(goals as f64 / 7.00) as i32),
            &line,
            &position,
        ));

        let shorthanded_points = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &(0..=(points as f64 / 7.00) as i32),
            &line,
            &position,
        ));

        let overtime_goals = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &(0..=(goals as f64 / 12.00) as i32),
            &line,
            &position,
        ));

        let game_winning_goals = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &(0..=(goals as f64 / 7.00) as i32),
            &line,
            &position,
        ));

        let time_on_ice_per_gp = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &TIME_ON_ICE_PER_GP,
            &line,
            &position,
        )) as u64;

        let shots_on_goal = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &SHOTS_ON_GOAL,
            &line,
            &position,
        ));

        let face_off_percentage = thread_rng().gen_range(adjust_range_by_line_and_pos(
            &FACE_OFF_PERCENTAGE,
            &line,
            &position,
        ));

        SkaterStats {
            games_played: thread_rng().gen_range(GAMES_PLAYED),
            goals,
            assists,
            points,
            plus_minus,
            penalty_minutes,
            powerplay_goals,
            powerplay_points,
            shorthanded_goals,
            shorthanded_points,
            overtime_goals,
            time_on_ice_per_gp: Duration::from_secs(time_on_ice_per_gp),
            game_winning_goals,
            shots_on_goal,
            face_off_percentage,
        }
    }
}
