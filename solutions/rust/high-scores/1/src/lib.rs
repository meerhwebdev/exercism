#[derive(Debug)]
pub struct HighScores {
    list:Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        let scores_list = scores.to_vec();
        HighScores { list:scores_list }
    }

    pub fn scores(&self) -> &[u32] {
        &self.list
    }

    pub fn latest(&self) -> Option<u32> {
        self.list.last().cloned()
    }
    
    pub fn personal_best(&self) -> Option<u32> {
        self.list.iter().cloned().max()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut list:Vec<u32> = self.list.clone();
        list.sort_by(|a, b| b.cmp(a));
        list.into_iter().take(3).collect()
    }
}