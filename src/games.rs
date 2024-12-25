use std::ops::RangeInclusive;

pub const OVERTIME: bool = false;
pub const SHOOTOUT: bool = false;

pub const GOALS: RangeInclusive<i32> = 0..=10;
pub const SHOTS_ON_GOAL: RangeInclusive<i32> = 20..=50;
pub const FACE_OFF_PERCENT: RangeInclusive<f64> = 40.0..=60.0;
pub const POWERPLAY_PERCENT: RangeInclusive<f64> = 0.0..=35.0;
pub const PENALTY_MINUTES: RangeInclusive<i32> = 0..=12;
pub const HITS: RangeInclusive<i32> = 20..=40;
pub const BLOCKED_SHOTS: RangeInclusive<i32> = 5..=15;
pub const GIVEAWAYS: RangeInclusive<i32> = 5..=20;
pub const TAKEAWAYS: RangeInclusive<i32> = 1..=5;
