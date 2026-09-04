use regex::{Regex, RegexBuilder};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SequenceSearchTarget {
    BiologicalSequence, // ignores gaps, e.g. ACG matches ACG, A-CG, AC--G, etc.
    AlignmentRow,       // doesn't - only A-CG matches A-CG
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MatchPosition {
    start_col: usize,
    end_col: usize,
}

impl MatchPosition {
    pub fn new(start_col: usize, end_col: usize) -> Self {
        MatchPosition { start_col, end_col }
    }

    pub fn start_col(&self) -> usize {
        self.start_col
    }

    pub fn end_col(&self) -> usize {
        self.end_col
    }
}

pub fn regex_match_positions(
    re: &Regex,
    seq: &str,
    target: SequenceSearchTarget,
) -> Vec<MatchPosition> {
    match target {
        SequenceSearchTarget::BiologicalSequence => regex_match_positions_gapaware(re, seq),
        SequenceSearchTarget::AlignmentRow => regex_match_positions_naive(re, seq),
    }
}

// A helper for ensuring correct case sensitivity when buliding regexes

pub fn compile_regex(pattern: &str, case_insensitive: bool) -> Result<Regex, regex::Error> {
    RegexBuilder::new(pattern)
        .case_insensitive(! case_insensitive)
        .build()
}

pub fn regex_match_positions_naive(re: &Regex, seq: &str) -> Vec<MatchPosition> {
    re.find_iter(seq)
        .map(|m| MatchPosition::new(m.start(), m.end()))
        .collect::<Vec<MatchPosition>>()
}

pub fn regex_match_positions_gapaware(re: &Regex, seq: &str) -> Vec<MatchPosition> {
    let gap_mapper = GapMapper::new(seq);
    let mut match_positions: Vec<MatchPosition> = Vec::new();
    for m in re.find_iter(&gap_mapper.degapped_seq) {
        if m.start() < m.end() {
            match_positions.push(MatchPosition::new(
                gap_mapper.map_back(m.start()),
                gap_mapper.map_back(m.end() - 1) + 1,
            ));
        }
    }

    match_positions
}

pub struct GapMapper {
    pub degapped_seq: String,
    orig_pos: Vec<usize>,
}

impl GapMapper {
    fn new(seq: &str) -> Self {
        let mut degapped_seq: Vec<char> = Vec::with_capacity(seq.len());
        let mut orig_pos: Vec<usize> = Vec::with_capacity(seq.len());
        for (i, c) in seq.as_bytes().iter().enumerate() {
            if *c != b'-' {
                degapped_seq.push(*c as char);
                orig_pos.push(i);
            }
        }
        GapMapper {
            degapped_seq: degapped_seq.into_iter().collect(),
            orig_pos,
        }
    }

    fn map_back(&self, pos: usize) -> usize {
        self.orig_pos[pos]
    }
}

#[cfg(test)]
mod tests {

    use regex::Regex;

    use crate::seq_match::{
        regex_match_positions_gapaware, regex_match_positions_naive, GapMapper, MatchPosition,
    };

    #[test]
    fn test_regex_match_position_naive() {
        let re = Regex::new("A[CT]G").unwrap();
        let seq = "AATGXACGY";
        let match_pos = regex_match_positions_naive(&re, seq);
        assert_eq!(
            match_pos,
            vec![
                MatchPosition {
                    start_col: 1,
                    end_col: 4
                },
                MatchPosition {
                    start_col: 5,
                    end_col: 8
                },
            ]
        );
    }

    #[test]
    fn test_gap_mapper() {
        let gm = GapMapper::new("-AA-YK--GQ--");
        // The degapped sequence is the original, but (surprise!) without gaps
        assert_eq!(gm.degapped_seq, "AAYKGQ");
        // Postion 0 in the degapped sequence (A) is position 1 in the original
        assert_eq!(gm.map_back(0), 1);
        // Postion 1 in the degapped sequence (A) is position 2 in the original
        assert_eq!(gm.map_back(1), 2);
        // etc...
        assert_eq!(gm.map_back(2), 4); // Y
        assert_eq!(gm.map_back(3), 5); // K
        assert_eq!(gm.map_back(4), 8); // G
        assert_eq!(gm.map_back(5), 9); // Q
    }

    #[test]
    fn test_regex_match_position_gapaware() {
        let re = Regex::new("A[CT]G").unwrap();
        let seq = "AATGXA-C-GY";
        let match_pos = regex_match_positions_gapaware(&re, seq);
        assert_eq!(
            match_pos,
            vec![
                MatchPosition {
                    start_col: 1,
                    end_col: 4
                },
                MatchPosition {
                    start_col: 5,
                    end_col: 10
                },
            ]
        );
    }

    #[test]
    fn test_regex_match_position_trail() {
        let re = Regex::new("MANES").unwrap();
        let seq = "MANES----H";
        let match_pos = regex_match_positions_gapaware(&re, seq);
        assert_eq!(
            match_pos,
            vec![MatchPosition {
                start_col: 0,
                end_col: 5
            },]
        );
    }

    #[test]
    fn test_regex_match_position_lead() {
        let re = Regex::new("MANES").unwrap();
        let seq = "--MANES";
        let match_pos = regex_match_positions_gapaware(&re, seq);
        assert_eq!(
            match_pos,
            vec![MatchPosition {
                start_col: 2,
                end_col: 7
            },]
        );
    }

    #[test]
    fn test_regex_match_position_lead_and_trail() {
        let re = Regex::new("MANES").unwrap();
        let seq = "--MANES----H";
        let match_pos = regex_match_positions_gapaware(&re, seq);
        assert_eq!(
            match_pos,
            vec![MatchPosition {
                start_col: 2,
                end_col: 7
            },]
        );
    }

    #[test]
    fn test_case_insensitive_naive() {
        use regex::RegexBuilder;
        let re = RegexBuilder::new("ACG")
            .case_insensitive(true)
            .build()
            .unwrap();
        let seq = "acgtACGtacg";

        let match_pos = regex_match_positions_naive(&re, seq);

        assert_eq!(
            match_pos,
            vec![
                MatchPosition {
                    start_col: 0,
                    end_col: 3
                },
                MatchPosition {
                    start_col: 4,
                    end_col: 7
                },
                MatchPosition {
                    start_col: 8,
                    end_col: 11
                },
            ]
        );
    }

    #[test]
    fn test_case_insensitive_gapaware() {
        use regex::RegexBuilder;
        let re = RegexBuilder::new("A[CT]G")
            .case_insensitive(true)
            .build()
            .unwrap();
        let seq = "a-c-g-ACG";

        let match_pos = regex_match_positions_gapaware(&re, seq);

        assert_eq!(
            match_pos,
            vec![
                MatchPosition {
                    start_col: 0,
                    end_col: 5
                },
                MatchPosition {
                    start_col: 6,
                    end_col: 9
                },
            ]
        );
    }

    #[test]
    fn test_case_sensitive_still_works() {
        let re = Regex::new("ACG").unwrap();
        let seq = "acgtACGtacg";

        let match_pos = regex_match_positions_naive(&re, seq);

        // Should only match uppercase ACG
        assert_eq!(
            match_pos,
            vec![MatchPosition {
                start_col: 4,
                end_col: 7
            },]
        );
    }
}
