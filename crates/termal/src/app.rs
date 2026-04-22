// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

use std::{collections::HashMap, fmt};

use regex::Regex;

use termal_alignment::alignment::{
    Alignment,
    RefSpec
};

use crate::{
    app::Metric::{PctIdWrtConsensus, SeqLen},
    app::SeqOrdering::{MetricDecr, MetricIncr, SourceFile, User},
};

#[derive(Clone, Copy)]
pub enum SeqOrdering {
    SourceFile,
    MetricIncr,
    MetricDecr,
    User,
}

impl fmt::Display for SeqOrdering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sord = match self {
            SourceFile => '-',
            MetricIncr => '↑',
            MetricDecr => '↓',
            User => 'u',
        };
        write!(f, "{}", sord)
    }
}

#[derive(Clone, Copy)]
pub enum Metric {
    PctIdWrtConsensus,
    SeqLen,
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metric = match self {
            PctIdWrtConsensus => "%id (cons)",
            SeqLen => "seq len",
        };
        write!(f, "{}", metric)
    }
}

pub enum SearchState {
    Header(HdrSearchState),
    Sequence(SeqSearchState),
}

pub struct HdrSearchState {
    // These will eventually be used, when we highlight the actual matching parts of the label, and
    // to allow more informative messages like "pattern 'xyz' has no match".
    #[allow(dead_code)]
    pub pattern: String,
    match_ranks: Vec<usize>,
    ordered_match_screenlinenums: Vec<usize>,
    screenlinenum_to_index: HashMap<usize, usize>,
    current_match_ndx: Option<usize>,
    current_match_rank: Option<usize>,
    // Whether a header matches or not; used when iterating over all headers and determining
    // whether to highlight or not. As many elements as there are sequences (and hence headers) in the alignment.
    hdr_match_status: Vec<bool>,
}

pub struct SeqSearchState {
    #[allow(dead_code)]
    pattern: String,
    matches_by_rank: HashMap<usize, Vec<MatchPosition>>,
    current: Option<(usize, usize)>, // (rank, match_ndx)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MatchPosition {
    start_col: usize,
    end_col: usize,
}

impl MatchPosition {
    pub fn start_col(&self) -> usize {
        self.start_col
    }

    pub fn end_col(&self) -> usize {
        self.end_col
    }
}

pub enum JumpTarget {
    HeaderLine(usize),
    Match(usize, MatchPosition),
}

#[derive(Clone)]
pub enum MessageKind {
    Info,
    Warning,
    Error,
    Debug,
    Argument,
}

// Simple, 1-line message (possibly just "")
pub struct CurrentMessage {
    pub prefix: String,
    pub message: String,
    pub kind: MessageKind,
}

pub struct App {
    pub filename: String,
    pub alignment: Alignment,
    ordering_criterion: SeqOrdering,
    metric: Metric,
    // Specifies in which order the aligned sequences should be displayed. The elements of this Vec
    // are _indices_ into the Vec's of headers and sequences that together make up the alignment.
    // By default, they are just ordered from 0 to num_seq - 1, but the user can choose to order
    // according to the current metric, in which case the ordering becomes that of the metric's
    // value for each sequence.
    pub ordering: Vec<usize>,
    pub reverse_ordering: Vec<usize>,
    user_ordering: Option<Vec<String>>,
    pub search_state: Option<SearchState>,
    current_msg: CurrentMessage,
}

impl App {
    pub fn new(path: &str, alignment: Alignment, usr_ord: Option<Vec<String>>) -> Self {
        let len = alignment.num_seq();
        let cur_msg = CurrentMessage {
            prefix: String::from(""),
            message: String::from(""),
            kind: MessageKind::Info,
        };
        let ordering_criterion = if usr_ord.is_some() {
            User
        } else {
            SourceFile
        };
        let mut app = App {
            filename: path.to_string(),
            alignment,
            ordering_criterion,
            metric: PctIdWrtConsensus,
            ordering: (0..len).collect(),
            reverse_ordering: (0..len).collect(),
            user_ordering: usr_ord,
            search_state: None,
            current_msg: cur_msg,
        };
        app.recompute_ordering();
        app
    }

    // Computed properties (TODO: could be set in a struct member, as they do not change)
    // FIXME where do we need num_seq as u16?

    pub fn num_seq(&self) -> u16 {
        self.alignment.num_seq().try_into().unwrap()
    }

    pub fn aln_len(&self) -> u16 {
        self.alignment.aln_len().try_into().unwrap()
    }

    fn recompute_ordering(&mut self) {
        match self.ordering_criterion {
            MetricIncr => {
                self.ordering = order(self.order_values());
            }
            MetricDecr => {
                let mut ord = order(self.order_values());
                ord.reverse();
                self.ordering = ord;
            }
            SourceFile => {
                self.ordering = (0..self.alignment.num_seq()).collect();
            }
            User => {
                // Do not change ordering if no user ordering provided, or if it had
                // problems (this is checked early on, in main(), around l. 180 (as of commit
                // 13a2e2e).).
                match &self.user_ordering {
                    None => {
                        // Note: self.ordering_criterion is not supposed to have value 'User' unless a
                        // valid ordering was supplied (see prev_ordering_criterion() and
                        // next_ordering_criterion()).
                    }
                    Some(uord_vec) => {
                        // Good ordering
                        // Technically, we could index by &str, but I'm not sure we'd gain a lot.
                        let mut hdr2rank: HashMap<String, usize> = HashMap::new();
                        for (idx, hdr) in self.alignment.headers.iter().enumerate() {
                            hdr2rank.insert(hdr.to_string(), idx);
                        }
                        // Iterate over ordering, looking up file index from the above hash.
                        let mut result: Vec<usize> = Vec::new();
                        // TODO: now that we no longer check for discrepancies here, this should be
                        //feasible in a sinmple map.
                        for hdr in uord_vec.iter() {
                            match hdr2rank.get(hdr) {
                                Some(rank) => result.push(*rank),
                                None => break,
                            }
                        }
                        self.ordering = result;
                    }
                }
            }
        }
        self.reverse_ordering = order(&self.ordering);
    }

    pub fn next_ordering_criterion(&mut self) {
        self.ordering_criterion = match self.ordering_criterion {
            SourceFile => MetricIncr,
            MetricIncr => MetricDecr,
            // move to User IFF valid ordering
            MetricDecr => match self.user_ordering {
                Some(_) => User,
                None => SourceFile,
            },
            User => SourceFile,
        };
        self.recompute_ordering();
        self.reorder_matches();
    }

    pub fn prev_ordering_criterion(&mut self) {
        self.ordering_criterion = match self.ordering_criterion {
            MetricIncr => SourceFile,
            MetricDecr => MetricIncr,
            User => MetricDecr,
            // move to User IFF valid ordering
            SourceFile => match self.user_ordering {
                Some(_) => User,
                None => MetricDecr,
            },
        };
        self.recompute_ordering();
        self.reorder_matches();
    }

    // Maps a rank (= index in the original alignment) to the corresponding line on the screen
    // (which may or may not be visible). This is affected by the ordering. This is NOT
    // user-facing, hence 0-based.
    pub fn rank_to_screenline(&self, rank: usize) -> usize {
        self.reverse_ordering[rank]
    }

    pub fn next_metric(&mut self) {
        self.metric = match self.metric {
            PctIdWrtConsensus => SeqLen,
            SeqLen => PctIdWrtConsensus,
        };
        self.recompute_ordering();
        self.reorder_matches();
    }

    // NOTE: for now, there are only two metrics, so next and prev are the same. This might change,
    // however.
    pub fn prev_metric(&mut self) {
        self.metric = match self.metric {
            PctIdWrtConsensus => SeqLen,
            SeqLen => PctIdWrtConsensus,
        };
        self.recompute_ordering();
        self.reorder_matches();
    }

    pub fn output_info(&self) {
        println!("name: {}", self.filename);
        println!("nb_sequences: {}", self.num_seq());
        println!("nb_columns: {}", self.aln_len());
        println!();
    }

    pub fn get_seq_ordering(&self) -> SeqOrdering {
        self.ordering_criterion
    }

    pub fn get_metric(&self) -> Metric {
        self.metric
    }

    // TODO: rename to order_by_metric
    pub fn order_values(&self) -> &Vec<f64> {
        match self.metric {
            PctIdWrtConsensus => &self.alignment.id_wrt_reference,
            SeqLen => &self.alignment.relative_seq_len,
        }
    }

    // Search

    pub fn regex_search_labels(&mut self, pattern: &str) {
        let try_re = Regex::new(pattern);
        match try_re {
            Ok(re) => {
                let matches: Vec<usize> = self
                    .alignment
                    .headers
                    .iter()
                    .enumerate()
                    .filter_map(|(i, line)| re.is_match(line).then_some(i))
                    .collect();

                // Start with all false, and flip to true only for matching lines
                let mut match_linenum_vec: Vec<bool> = vec![false; self.alignment.num_seq()];
                for i in &matches {
                    match_linenum_vec[*i] = true;
                }

                let ordered_matches = self.ordered_hdr_matches(&matches);
                let mut l2ndx: HashMap<usize, usize> = HashMap::new();
                for (ndx, l) in ordered_matches.iter().enumerate() {
                    l2ndx.insert(*l, ndx);
                }

                let mut cur_match_ndx: Option<usize> = None;
                let mut cur_match_rank: Option<usize> = None;
                if !matches.is_empty() {
                    cur_match_ndx = Some(0);
                    let current_match_scrnln = ordered_matches[cur_match_ndx.unwrap()];
                    cur_match_rank = Some(self.ordering[current_match_scrnln]);
                }

                self.search_state = Some(SearchState::Header(HdrSearchState {
                    pattern: String::from(pattern),
                    match_ranks: matches,
                    ordered_match_screenlinenums: ordered_matches,
                    screenlinenum_to_index: l2ndx,
                    current_match_ndx: cur_match_ndx,
                    current_match_rank: cur_match_rank,
                    hdr_match_status: match_linenum_vec,
                }));
            }
            Err(e) => {
                self.error_msg(format!("Malformed regex {}.", e));
                self.search_state = None;
            }
        }
    }

    fn next_match_expect(
        reverse_ordering: &Vec<usize>,
        ordering: &Vec<usize>,
        num_seq: usize,
        matches_by_rank: &HashMap<usize, Vec<MatchPosition>>,
        cur_match: (usize, usize),
    ) -> (usize, usize) {
        let (rank, mut match_ndx) = cur_match;
        let current_seq_matches = matches_by_rank.get(&rank).unwrap();
        match_ndx += 1;
        if match_ndx < current_seq_matches.len() {
            (rank, match_ndx)
        } else {
            let mut current_screen_line = reverse_ordering[rank];
            loop {
                current_screen_line = (current_screen_line + 1).rem_euclid(num_seq);
                let current_rank = ordering[current_screen_line];
                if matches_by_rank.contains_key(&current_rank) {
                    break (current_rank, 0);
                }
            }
        }
    }

    fn previous_match_expect(
        reverse_ordering: &Vec<usize>,
        ordering: &Vec<usize>,
        num_seq: usize,
        matches_by_rank: &HashMap<usize, Vec<MatchPosition>>,
        cur_match: (usize, usize),
    ) -> (usize, usize) {
        let (rank, mut match_ndx) = cur_match;
        if match_ndx > 0 {
            match_ndx -= 1;
            (rank, match_ndx)
        } else {
            let mut current_screen_line = reverse_ordering[rank];
            loop {
                current_screen_line =
                    (current_screen_line as isize - 1).rem_euclid(num_seq as isize) as usize;
                let current_rank = ordering[current_screen_line];
                if matches_by_rank.contains_key(&current_rank) {
                    let prev_seq_matches = matches_by_rank.get(&current_rank).unwrap();
                    break (current_rank, prev_seq_matches.len() - 1);
                }
            }
        }
    }

    pub fn increment_current_match(&mut self, count: isize) {
        if let Some(state) = &mut self.search_state {
            match state {
                SearchState::Header(hdr_state) => {
                    let nb_matches = hdr_state.ordered_match_screenlinenums.len();
                    if nb_matches > 0 {
                        let new_match_ndx = (hdr_state.current_match_ndx.unwrap() as isize + count)
                            .rem_euclid(nb_matches as isize)
                            as usize;
                        hdr_state.current_match_ndx = Some(new_match_ndx);
                        let new_match_scrnln =
                            hdr_state.ordered_match_screenlinenums[new_match_ndx];
                        let new_match_rank = self.ordering[new_match_scrnln];
                        hdr_state.current_match_rank = Some(new_match_rank);
                    }
                }
                SearchState::Sequence(seq_state) => {
                    let step_count = count.unsigned_abs();
                    for _ in 0..step_count {
                        match seq_state.current {
                            Some((rank, match_ndx)) => {
                                seq_state.current = Some(if count >= 0 {
                                    Self::next_match_expect(
                                        &self.reverse_ordering,
                                        &self.ordering,
                                        self.alignment.sequences.len(),
                                        &seq_state.matches_by_rank,
                                        (rank, match_ndx),
                                    )
                                } else {
                                    Self::previous_match_expect(
                                        &self.reverse_ordering,
                                        &self.ordering,
                                        self.alignment.sequences.len(),
                                        &seq_state.matches_by_rank,
                                        (rank, match_ndx),
                                    )
                                });
                            }
                            None => break,
                        }
                    }
                }
            }
        }
    }

    pub fn current_match(&self) -> Option<JumpTarget> {
        if let Some(state) = &self.search_state {
            match state {
                SearchState::Header(hdr_state) => {
                    if !hdr_state.ordered_match_screenlinenums.is_empty() {
                        Some(JumpTarget::HeaderLine(
                            hdr_state.ordered_match_screenlinenums
                                [hdr_state.current_match_ndx.unwrap()],
                        ))
                    } else {
                        None
                    }
                }
                SearchState::Sequence(seq_state) => {
                    if let Some((rank, match_ndx)) = seq_state.current {
                        let screen_line = self.reverse_ordering[rank];
                        let match_pos = seq_state.matches_by_rank[&rank][match_ndx];
                        Some(JumpTarget::Match(screen_line, match_pos))
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn display_current_match(&mut self) {
        match &self.search_state {
            Some(SearchState::Header(hdr_state)) => {
                let nb_matches = hdr_state.ordered_match_screenlinenums.len();
                if nb_matches > 0 {
                    self.info_msg(format!(
                        "match #{}/{}",
                        hdr_state.current_match_ndx.unwrap() + 1,
                        nb_matches
                    ));
                } else {
                    self.info_msg("No match.");
                }
            }
            Some(SearchState::Sequence(seq_state)) => {
                let nb_matches: usize = seq_state
                    .matches_by_rank
                    .values()
                    .map(|matches| matches.len())
                    .sum();

                if nb_matches > 0 {
                    if let Some((rank, match_ndx)) = seq_state.current {
                        let mut current_match_num = 0;
                        for cur_rank in &self.ordering {
                            if let Some(matches) = seq_state.matches_by_rank.get(&cur_rank) {
                                if *cur_rank != rank {
                                    current_match_num += matches.len();
                                } else {
                                    current_match_num += match_ndx + 1;
                                    break;
                                }
                            }
                        }
                        self.info_msg(format!("match #{}/{}", current_match_num, nb_matches));
                    } else {
                        self.info_msg("No match.");
                    }
                } else {
                    self.info_msg("No match.");
                }
            }
            None => self.info_msg("No current search."),
        }
    }

    pub fn screenline_is_hdr_match(&self, screenline: usize) -> bool {
        if let Some(SearchState::Header(state)) = &self.search_state {
            state.hdr_match_status[self.ordering[screenline]]
        } else {
            false
        }
    }

    pub fn screenline_is_current_hdr_match(&self, screenline: usize) -> bool {
        if let Some(SearchState::Header(state)) = &self.search_state {
            if let Some(rank) = state.current_match_rank {
                return self.ordering[screenline] == rank;
            }
        }
        false
    }

    pub fn cell_is_seq_match(&self, screenline: usize, col: usize) -> bool {
        if let Some(SearchState::Sequence(state)) = &self.search_state {
            let rank = self.ordering[screenline];
            if let Some(matches) = state.matches_by_rank.get(&rank) {
                return matches.iter().any(|m| m.start_col <= col && col < m.end_col);
            }
        }
        false
    }

    pub fn cell_is_current_seq_match(&self, screenline: usize, col: usize) -> bool {
        if let Some(SearchState::Sequence(state)) = &self.search_state {
            if let Some((rank, match_ndx)) = state.current {
                if self.ordering[screenline] == rank {
                    let m = state.matches_by_rank[&rank][match_ndx];
                    return m.start_col <= col && col < m.end_col;
                }
            }
        }
        false
    }

    pub fn reset_lbl_search(&mut self) {
        self.search_state = None;
    }

    pub fn regex_search_seq(&mut self, pattern: &str) {
        let try_re = Regex::new(pattern);
        match try_re {
            Ok(re) => {
                let mut matches_by_rank: HashMap<usize, Vec<MatchPosition>> = HashMap::new();
                for (rank, sequence) in self.alignment.sequences.iter().enumerate() {
                    let seq_matches = re
                        .find_iter(sequence)
                        .map(|m| MatchPosition {
                            start_col: m.start(),
                            end_col: m.end(),
                        })
                        .collect::<Vec<MatchPosition>>();
                    if !seq_matches.is_empty() {
                        matches_by_rank.insert(rank, seq_matches);
                    }
                }
                let current = self.first_seq_match_in_order(&matches_by_rank);
                self.search_state = Some(SearchState::Sequence(SeqSearchState {
                    pattern: String::from(pattern),
                    matches_by_rank,
                    current,
                }));
            }
            Err(e) => {
                self.error_msg(format!("Malformed regex {}.", e));
                self.search_state = None;
            }
        }
    }

    fn ordered_hdr_matches(&self, match_ranks: &Vec<usize>) -> Vec<usize> {
        let mut screenlines: Vec<usize> = match_ranks
            .iter()
            .map(|r| self.reverse_ordering[*r])
            .collect();
        screenlines.sort();
        screenlines
    }

    fn first_seq_match_in_order(
        &self,
        matches_by_rank: &HashMap<usize, Vec<MatchPosition>>,
    ) -> Option<(usize, usize)> {
        self.ordering
            .iter()
            .find(|rank| matches_by_rank.contains_key(rank))
            .map(|rank| (*rank, 0))
    }

    fn reorder_matches(&mut self) {
        match &mut self.search_state {
            Some(SearchState::Header(state)) => {
                if !state.match_ranks.is_empty() {
                    state.ordered_match_screenlinenums = state
                        .match_ranks
                        .iter()
                        .map(|r| self.reverse_ordering[*r])
                        .collect();
                    state.ordered_match_screenlinenums.sort();
                    state.screenlinenum_to_index.clear();
                    for (ndx, line) in state.ordered_match_screenlinenums.iter().enumerate() {
                        state.screenlinenum_to_index.insert(*line, ndx);
                    }
                    let current_match_scrnln = self.reverse_ordering[state.current_match_rank.unwrap()];
                    state.current_match_ndx = Some(state.screenlinenum_to_index[&current_match_scrnln]);
                    let mut matching_scrln = vec![false; self.alignment.num_seq()];
                    for line in &state.ordered_match_screenlinenums {
                        matching_scrln[*line] = true;
                    }
                    state.hdr_match_status = vec![false; self.alignment.num_seq()];
                    for line in &state.ordered_match_screenlinenums {
                        let rank = self.ordering[*line];
                        state.hdr_match_status[rank] = true;
                    }
                }
            }
            Some(SearchState::Sequence(_)) => {}
            None => {}
        }
    }

    // Messages

    pub fn current_message(&self) -> &CurrentMessage {
        &self.current_msg
    }

    pub fn clear_msg(&mut self) {
        self.current_msg = CurrentMessage {
            prefix: String::from(""),
            message: String::from(""),
            kind: MessageKind::Info,
        }
    }

    pub fn info_msg(&mut self, msg: impl Into<String>) {
        self.current_msg = CurrentMessage {
            prefix: String::from(""),
            message: msg.into(),
            kind: MessageKind::Info,
        };
    }

    pub fn warning_msg(&mut self, msg: impl Into<String>) {
        self.current_msg = CurrentMessage {
            prefix: String::from("WARNING: "),
            message: msg.into(),
            kind: MessageKind::Warning,
        };
    }

    pub fn error_msg(&mut self, msg: impl Into<String>) {
        self.current_msg = CurrentMessage {
            prefix: String::from("ERROR: "),
            message: msg.into(),
            kind: MessageKind::Error,
        };
    }

    pub fn debug_msg(&mut self, msg: impl Into<String>) {
        self.current_msg = CurrentMessage {
            prefix: String::from(""),
            message: msg.into(),
            kind: MessageKind::Debug,
        };
    }

    pub fn argument_msg(&mut self, pfx: impl Into<String>, msg: impl Into<String>) {
        self.current_msg = CurrentMessage {
            prefix: pfx.into(),
            message: msg.into(),
            kind: MessageKind::Argument,
        };
    }

    pub fn add_argument_char(&mut self, c: char) {
        self.current_msg.message.push(c);
        self.current_msg.kind = MessageKind::Argument;
    }

    pub fn pop_argument_char(&mut self) {
        self.current_msg.message.pop();
        self.current_msg.kind = MessageKind::Argument;
    }

    pub fn set_aln_ref(&mut self, ref_spec_str: &str) {
        if ref_spec_str.is_empty() {
            self.alignment.set_ref_spec(RefSpec::Consensus);
        } else {
            let rank: usize = ref_spec_str.parse().expect("Malformed integer");
            self.alignment.set_ref_spec(RefSpec::Rank(rank));
        }
    }
}

// Computes an ordering WRT an array, that is, an array of indices of elements of the source array,
// after sorting. Eg [3, -2, 7] -> [1, 0, 2], because the smalllest element has index 1, the next
// has index 0, and the largest has index 2 (in the original array).
fn order<T: PartialOrd>(elems: &[T]) -> Vec<usize> {
    // let result: Vec<usize> = Vec::with_capacity(elems.len());
    let init_order: Vec<usize> = (0..elems.len()).collect();
    let zip_iter = init_order.iter().zip(elems);
    let mut unsorted_pairs: Vec<(&usize, &T)> = zip_iter.collect();
    unsorted_pairs.sort_by(|(_, t1), (_, t2)| t1.partial_cmp(t2).expect("Unorder!"));
    unsorted_pairs
        .into_iter()
        .map(|(u, _)| *u)
        .collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {

    use termal_alignment::Alignment;

    use crate::{
        app::{order, App, SearchState},
    };

    #[test]
    fn test_order_00() {
        assert_eq!(vec![2, 1, 0], order(&vec![20.0, 15.0, 10.0]));
    }

    #[test]
    fn test_order_05() {
        assert_eq!(
            vec![3, 2, 0, 1, 4],
            order(&vec![12.23, 34.89, 7.0, -23.2, 100.0]),
        );
    }

    #[test]
    fn test_order_10() {
        // Reverse order
        let orig = vec![3.0, 2.0, 5.0, 1.0, 4.0];
        let direct_order = order(&orig);
        assert_eq!(vec![3, 1, 0, 4, 2], direct_order);
        let reverse_order = order(&direct_order);
        assert_eq!(vec![2, 1, 4, 0, 3], reverse_order);
    }

    #[test]
    fn test_ordering_00() {
        let hdrs = vec![
            String::from("R1"),
            String::from("R2"),
            String::from("R3"),
            String::from("R4"),
        ];
        let seqs = vec![
            String::from("catgcatatg"), // 0 diffs WRT consensus
            String::from("cCtgcatatg"), // 1 diffs WRT consensus
            String::from("catAcTtatg"), // 2 diffs WRT consensus
            String::from("caGgAataAg"), // 3 diffs WRT consensus
        ];
        let aln = Alignment::from_vecs(hdrs, seqs);
        let mut app = App::new("TEST", aln, None);
        assert_eq!(app.ordering, vec![0, 1, 2, 3]);
        app.next_ordering_criterion();
        // Ordering is now by increasing metric, which is (by default) %id WRT consensus. Given the above
        // sequences, this effectively reverses the order.
        assert_eq!(app.ordering, vec![3, 2, 1, 0]);
        app.next_ordering_criterion();
        // Now by decreasing metric, which in this case is (by construction) the same as the
        // original.
        assert_eq!(app.ordering, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_ordering_05() {
        let hdrs = vec![
            String::from("R1"),
            String::from("R2"),
            String::from("R3"),
            String::from("R4"),
            String::from("R5"),
        ];
        let seqs = vec![
            String::from("catgcatatg"), // 0 diffs WRT consensus
            String::from("caGgAaCaAg"), // 4 diffs WRT consensus
            String::from("catAcTtatg"), // 2 diffs WRT consensus
            String::from("cCtgcatatg"), // 1 diffs WRT consensus
            String::from("caGgAataAg"), // 3 diffs WRT consensus
        ];
        let aln = Alignment::from_vecs(hdrs, seqs);
        let mut app = App::new("TEST", aln, None);
        assert_eq!(app.ordering, vec![0, 1, 2, 3, 4]);
        app.next_ordering_criterion();
        assert_eq!(app.ordering, vec![1, 4, 2, 3, 0]);
        assert_eq!(app.reverse_ordering, vec![4, 0, 2, 3, 1]);
        app.next_ordering_criterion();
        assert_eq!(app.ordering, vec![0, 3, 2, 4, 1]);
        assert_eq!(app.reverse_ordering, vec![0, 4, 2, 1, 3]);
    }

    #[test]
    fn test_rank_to_screenline_00() {
        let hdrs = vec![
            String::from("R1"),
            String::from("R2"),
            String::from("R3"),
            String::from("R4"),
            String::from("R5"),
        ];
        let seqs = vec![
            String::from("catgcatatg"), // 0 diffs WRT consensus
            String::from("caGgAaCaAg"), // 4 diffs WRT consensus
            String::from("catAcTtatg"), // 2 diffs WRT consensus
            String::from("cCtgcatatg"), // 1 diffs WRT consensus
            String::from("caGgAataAg"), // 3 diffs WRT consensus
        ];
        let aln = Alignment::from_vecs(hdrs, seqs);
        let mut app = App::new("TEST", aln, None);
        assert_eq!(app.ordering, vec![0, 1, 2, 3, 4]);
        // Until ordering changes, rank == screenline
        assert_eq!(0, app.rank_to_screenline(0));
        assert_eq!(1, app.rank_to_screenline(1));
        assert_eq!(2, app.rank_to_screenline(2));
        assert_eq!(3, app.rank_to_screenline(3));
        assert_eq!(4, app.rank_to_screenline(4));
        app.next_ordering_criterion();
        assert_eq!(app.ordering, vec![1, 4, 2, 3, 0]);
        assert_eq!(app.reverse_ordering, vec![4, 0, 2, 3, 1]);
        // Now the ordering is by metric, so rank != screenline
        assert_eq!(app.rank_to_screenline(0), 4);
        assert_eq!(app.rank_to_screenline(1), 0);
        assert_eq!(app.rank_to_screenline(2), 2);
        assert_eq!(app.rank_to_screenline(3), 3);
        assert_eq!(app.rank_to_screenline(4), 1);
    }

    #[test]
    fn test_regex_lbl_search_10() {
        let hdrs = vec![
            String::from("Accipiter"),
            String::from("Aquila"),
            String::from("Milvus"),
            String::from("Buteo"),
            String::from("Pernis"),
        ];
        let seqs = vec![
            String::from("catgcatatg"), // 0 diffs WRT consensus
            String::from("caGgAaCaAg"), // 4 diffs WRT consensus
            String::from("catAcTtatg"), // 2 diffs WRT consensus
            String::from("cCtgcatatg"), // 1 diffs WRT consensus
            String::from("caGgAataAg"), // 3 diffs WRT consensus
        ];
        let aln = Alignment::from_vecs(hdrs, seqs);
        let mut app = App::new("TEST", aln, None);
        app.regex_search_labels("^A");
        match app.search_state {
            Some(SearchState::Header(state)) => {
                assert_eq!(state.pattern, "^A");
                assert_eq!(state.match_ranks, vec![0, 1]);
                assert_eq!(state.ordered_match_screenlinenums, vec![0, 1]);
                assert_eq!(state.current_match_ndx, Some(0));
                assert_eq!(state.current_match_rank, Some(0));
                assert_eq!(
                    state.hdr_match_status,
                    vec![true, true, false, false, false]
                );
            }
            Some(SearchState::Sequence(_)) => panic!(),
            None => panic!(),
        }
    }
}
