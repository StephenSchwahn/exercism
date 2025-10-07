#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { scores: scores }
    }

    pub fn scores(&self) -> &'a [u32] {
        self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.scores
            .iter()
            .reduce(|acc, curr| if curr > acc { curr } else { acc })
            .copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut top_three = vec![];
        for score in self.scores {
            top_three.push(*score);
            top_three.sort_unstable_by(|a, b| b.cmp(a));
            top_three.truncate(3)
        }
        top_three
    }
}
