use rand::{self, thread_rng, Rng};
use std::ops::RangeInclusive;

pub const GAMES: i32 = 82;
pub const WINS: RangeInclusive<i32> = 20..=54;
pub const LOSSES: RangeInclusive<i32> = 20..=54;
pub const OVERTIME_WINS: RangeInclusive<i32> = 0..=10;
pub const OVERTIME_LOSSES: RangeInclusive<i32> = 4..=16;
pub const POINTS: RangeInclusive<i32> = 47..=114;
pub const GOALS_FOR: RangeInclusive<i32> = 178..=302;
pub const GOALS_AGAINST: RangeInclusive<i32> = 198..=326;
pub const STREAK: [&str; 10] = ["L5", "L4", "L3", "L2", "L1", "W1", "W2", "W3", "W4", "W5"];

pub fn get_home_away_record(wins: i32, losses: i32, overtime_losses: i32) -> (String, String) {
    let home_wins = (wins as f64 * 0.7) as i32;
    let home_losses = (losses as f64 * 0.3) as i32;
    let home_otl = (overtime_losses as f64 * 0.2) as i32;

    let away_wins = wins - home_wins;
    let away_losses = losses - home_losses;
    let away_otl = overtime_losses - home_otl;

    let home_record = vec![
        home_wins.to_string(),
        home_losses.to_string(),
        home_otl.to_string(),
    ]
    .join("-");

    let away_record = vec![
        away_wins.to_string(),
        away_losses.to_string(),
        away_otl.to_string(),
    ]
    .join("-");

    (home_record, away_record)
}

pub fn get_shootout_record(overtime_wins: i32, overtime_losses: i32) -> String {
    let shootout_wins = if overtime_wins == 0 {
        0
    } else {
        (overtime_wins as f64 * 0.3) as i32
    };

    let shootout_losses = (overtime_losses as f64 * 0.1) as i32;

    vec![shootout_wins.to_string(), shootout_losses.to_string()].join("-")
}

pub fn get_last_10() -> String {
    let win = thread_rng().gen_range(1..=5);
    let otl = thread_rng().gen_range(0..=2);

    let lose = 10 - win - otl;

    vec![win.to_string(), lose.to_string(), otl.to_string()].join("-")
}