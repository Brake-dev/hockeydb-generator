use crate::goalies::Goalie;
use crate::skaters::Skater;
use crate::team_names::{CONFERENCE, DIVISION, TEAM_IDS};

#[derive(Debug, Clone)]
pub struct Team {
    pub team_id: String,
    pub team_name: String,
    pub conference: String,
    pub division: String,
    pub lines: Vec<Vec<Skater>>,
    pub goalies: Vec<Goalie>,
}

impl Team {
    pub fn new(team_name: String, index: usize) -> Team {
        let conference = if index < 16 {
            CONFERENCE[0].to_string()
        } else {
            CONFERENCE[1].to_string()
        };

        let division = if index < 8 {
            DIVISION[0].to_string()
        } else if index >= 8 && index < 16 {
            DIVISION[1].to_string()
        } else if index >= 16 && index < 24 {
            DIVISION[2].to_string()
        } else {
            DIVISION[3].to_string()
        };

        Team {
            team_id: TEAM_IDS[index].to_string(),
            team_name,
            conference,
            division,
            lines: vec![vec![], vec![], vec![], vec![]],
            goalies: vec![],
        }
    }
}
