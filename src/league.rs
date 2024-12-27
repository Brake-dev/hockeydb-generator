use crate::seasons::Season;

#[derive(Debug)]
pub struct League {
    pub seasons: Vec<Season>,
}

impl League {
    pub fn new() -> League {
        League { seasons: vec![] }
    }

    pub fn get_prev_season(&self, current_season_index: usize) -> Option<Season> {
        if current_season_index > 0 {
            let prev_season_index = current_season_index - 1;
            return Some(self.seasons[prev_season_index].clone());
        }

        None
    }
}
