#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores { scores: Vec::from(scores) }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        if let Some(x) = self.scores.last() {
            return Some(*x)
        }
        None
    }

    pub fn personal_best(&self) -> Option<u32> {
        let sorted_scores = self.sort_asc();
        let highest = HighScores::new(sorted_scores.as_slice());
        highest.latest()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let sorted_scores = self.sort_desc();
        let mut range: usize = 3;
        
        if sorted_scores.len() < range {
            range = sorted_scores.len();
        }
        
        let slice = &sorted_scores[0..range];
        Vec::from(slice)
    }

    fn sort_asc(&self) -> Vec<u32> {
        let mut sorted_scores = Vec::from(self.scores.as_slice());
        sorted_scores.sort();
        sorted_scores
    }

    fn sort_desc(&self) -> Vec<u32> {
        let mut sorted_scores = Vec::from(self.scores.as_slice());
        sorted_scores.sort_by(|a,b|b.partial_cmp(a).unwrap());
        sorted_scores
    }
}
