use std::ops::RangeInclusive;
use std::time::Duration;

pub const GAMES_PLAYED: RangeInclusive<i32> = 1..=82;
pub const GOALS: RangeInclusive<i32> = 0..=70;
pub const ASSISTS: RangeInclusive<i32> = 0..=100;
pub const POINTS: RangeInclusive<i32> = 0..=150;

pub const PLUS_MINUS: RangeInclusive<i32> = -60..=60;
pub const PENALTY_MINUTES: RangeInclusive<i32> = 0..=150;
pub const POWERPLAY_GOALS: RangeInclusive<i32> = 0..=30;
pub const POWERPLAY_POINTS: RangeInclusive<i32> = 0..=35;
pub const SHORTHANDED_GOALS: RangeInclusive<i32> = 0..=10;
pub const SHORTHANDED_POINTS: RangeInclusive<i32> = 0..=15;
pub const OVERTIME_GOALS: RangeInclusive<i32> = 0..=5;

pub const TIME_ON_ICE_PER_GP: RangeInclusive<Duration> =
    Duration::from_secs(120)..=Duration::from_secs(1500); // 2:00 - 25:00

pub const GAME_WINNING_GOALS: RangeInclusive<i32> = 0..=12;
pub const SHOTS_ON_GOAL: RangeInclusive<i32> = 0..=400;
pub const FACE_OFF_PERCENTAGE: RangeInclusive<i32> = 0..=100;
