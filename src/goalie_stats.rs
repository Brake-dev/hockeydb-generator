use std::ops::RangeInclusive;

pub const GAMES_PLAYED: RangeInclusive<i32> = 1..=64;
pub const GAMES_STARTED: RangeInclusive<i32> = 0..=64;
pub const WINS: RangeInclusive<i32> = 0..=38;
pub const LOSSES: RangeInclusive<i32> = 0..=31;
pub const OVERTIME_LOSSES: RangeInclusive<i32> = 0..=10;
pub const SHOTS_AGAINST: RangeInclusive<i32> = 20..=1845;
pub const GOALS_AGAINST_AVERAGE: RangeInclusive<f64> = 2.00..=5.00;
pub const SAVE_PERCENTAGE: RangeInclusive<f64> = 0.870..=0.920;
pub const SHUTOUT: RangeInclusive<i32> = 0..=6;
pub const GOALS: RangeInclusive<i32> = 0..=1;
pub const ASSISTS: RangeInclusive<i32> = 0..=5;
pub const PENALTY_MINUTES: RangeInclusive<i32> = 0..=14;
pub const TIME_ON_ICE: RangeInclusive<i32> = 15..=3630;
