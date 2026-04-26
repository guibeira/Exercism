#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        Self {
            scores: scores.into(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        let mut score = self.scores.clone();
        score.sort();
        score.reverse();
        score.first().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut score = self.scores.clone();
        score.sort();
        score.reverse();
        let mut result = vec![];
        for s in score.iter() {
            result.push(*s);
            if result.len() == 3 {
                break;
            }
        }
        result
    }
}
