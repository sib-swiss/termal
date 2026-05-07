use regex::Regex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MatchPosition {
    start_col: usize,
    end_col: usize,
}

impl MatchPosition {

    pub fn new(start_col: usize, end_col: usize) -> Self {
        MatchPosition {
            start_col,
            end_col,
        }
    }

    pub fn start_col(&self) -> usize {
        self.start_col
    }

    pub fn end_col(&self) -> usize {
        self.end_col
    }
}

fn regex_match_positions_naive(re: Regex, seq: &str) -> Vec<MatchPosition> {
    re.find_iter(seq)
    .map(|m| MatchPosition::new (
        m.start(), m.end()
    )).collect::<Vec<MatchPosition>>()
}

#[cfg(test)]
mod tests {

    use regex::Regex;

    use crate::seq_match::{
        MatchPosition,
        regex_match_positions,
    };

    #[test]
    fn test_regex_match_position_naive() {
        let re = Regex::new("A[CT]G").unwrap();
        let seq = "AATGXACGY";
        let match_pos = regex_match_positions_naive(re, seq);
        assert_eq!(match_pos,
            vec![
                MatchPosition{start_col: 1, end_col: 4},
                MatchPosition{start_col: 5, end_col: 8},
            ]);
    }
}

