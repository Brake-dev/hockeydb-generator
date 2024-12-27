use crate::season_names::{SEASON_IDS, SEASON_NUMS};
use crate::teams::Team;

#[derive(Debug, Clone)]
pub struct Season {
    pub season_id: String,
    pub season_name: String,
    pub season_num: i32,
    pub teams: Vec<Team>,
}

impl Season {
    pub fn new(season_name: String, index: usize) -> Season {
        Season {
            season_id: SEASON_IDS[index].to_string(),
            season_name,
            season_num: SEASON_NUMS[index],
            teams: vec![],
        }
    }
}
