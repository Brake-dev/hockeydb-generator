use crate::seasons::Season;

#[derive(Debug)]
pub struct League {
    pub seasons: Vec<Season>,
}

impl League {
    pub fn new() -> League {
        League { seasons: vec![] }
    }
}
