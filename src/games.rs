use rand::{thread_rng, Rng};
use std::ops::RangeInclusive;
use uuid::Uuid;

pub const GOALS: RangeInclusive<i32> = 0..=10;
pub const SHOTS_ON_GOAL: RangeInclusive<i32> = 20..=50;
pub const FACE_OFF_PERCENT: RangeInclusive<f32> = 40.0..=60.0;
pub const POWERPLAY_PERCENT: RangeInclusive<f32> = 0.0..=35.0;
pub const PENALTY_MINUTES: RangeInclusive<i32> = 0..=12;
pub const HITS: RangeInclusive<i32> = 20..=40;
pub const BLOCKED_SHOTS: RangeInclusive<i32> = 5..=15;
pub const GIVEAWAYS: RangeInclusive<i32> = 5..=20;
pub const TAKEAWAYS: RangeInclusive<i32> = 1..=5;

// Within division 	                    4 games × 5 opponents + 3 games × 2 opponents = 26
// Within conference, non-divisional 	3 games × 8 opponents = 24
// Inter-conference 	                2 games × 16 opponents = 32

pub struct Game {
    pub game_id: String,
    pub season_id: String,
    pub overtime: bool,
    pub shootout: bool,
    pub home_team_id: String,
    pub away_team_id: String,
    pub home_goals: i32,
    pub home_shots_on_goal: i32,
    pub home_face_off_percentage: f32,
    pub home_powerplay_percentage: f32,
    pub home_penalty_minutes: i32,
    pub home_hits: i32,
    pub home_blocked_shots: i32,
    pub home_giveaways: i32,
    pub home_takeaways: i32,
    pub away_goals: i32,
    pub away_shots_on_goal: i32,
    pub away_face_off_percentage: f32,
    pub away_powerplay_percentage: f32,
    pub away_penalty_minutes: i32,
    pub away_hits: i32,
    pub away_blocked_shots: i32,
    pub away_giveaways: i32,
    pub away_takeaways: i32,
}

impl Game {
    pub fn new(season_id: &String, home_team_id: &String, away_team_id: &String) -> Game {
        let overtime_percent = thread_rng().gen_range(0..=100);
        let overtime = overtime_percent <= 5;

        let shootout_percent = thread_rng().gen_range(0..=100);
        let shootout = overtime && shootout_percent <= 50;

        let home_goals = thread_rng().gen_range(GOALS);
        let home_shots_on_goal = thread_rng().gen_range(SHOTS_ON_GOAL);
        let home_face_off_percentage = thread_rng().gen_range(FACE_OFF_PERCENT);
        let home_powerplay_percentage = thread_rng().gen_range(POWERPLAY_PERCENT);
        let home_penalty_minutes = thread_rng().gen_range(PENALTY_MINUTES);
        let home_hits = thread_rng().gen_range(HITS);
        let home_blocked_shots = thread_rng().gen_range(BLOCKED_SHOTS);
        let home_giveaways = thread_rng().gen_range(GIVEAWAYS);
        let home_takeaways = thread_rng().gen_range(TAKEAWAYS);

        let away_goals = thread_rng().gen_range(GOALS);
        let away_shots_on_goal = thread_rng().gen_range(SHOTS_ON_GOAL);
        let away_face_off_percentage = thread_rng().gen_range(FACE_OFF_PERCENT);
        let away_powerplay_percentage = thread_rng().gen_range(POWERPLAY_PERCENT);
        let away_penalty_minutes = thread_rng().gen_range(PENALTY_MINUTES);
        let away_hits = thread_rng().gen_range(HITS);
        let away_blocked_shots = thread_rng().gen_range(BLOCKED_SHOTS);
        let away_giveaways = thread_rng().gen_range(GIVEAWAYS);
        let away_takeaways = thread_rng().gen_range(TAKEAWAYS);

        Game {
            game_id: String::from(Uuid::new_v4()),
            season_id: season_id.to_string(),
            overtime,
            shootout,
            home_team_id: home_team_id.to_string(),
            away_team_id: away_team_id.to_string(),
            home_goals,
            home_shots_on_goal,
            home_face_off_percentage,
            home_powerplay_percentage,
            home_penalty_minutes,
            home_hits,
            home_blocked_shots,
            home_giveaways,
            home_takeaways,
            away_goals,
            away_shots_on_goal,
            away_face_off_percentage,
            away_powerplay_percentage,
            away_penalty_minutes,
            away_hits,
            away_blocked_shots,
            away_giveaways,
            away_takeaways,
        }
    }
}
