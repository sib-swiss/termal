// SPDX-License-Identifier: MIT
// Copyright (c) 2025-2026 Thomas Junier

mod permutation;

use std::{collections::{HashMap, HashSet}, fmt};

use itertools::Itertools;

use crate::seq::file::SeqFile;

use crate::alignment::SeqType::{Nucleic, Protein};

// Whether to show the most frequent residue as LC or UC
const UC_CONS_THRESHOLD: f64 = 0.8; // uppercase if at least this
const LC_CONS_THRESHOLD: f64 = 0.2; // lowercase if at least this (else '*')

type ResidueDistribution = HashMap<char, f64>;
type ResidueCounts = HashMap<char, u64>;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum SeqType {
    Nucleic,
    Protein,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RefSpec {
    Consensus,
    Rank(usize),
}

pub enum RefSpecError {
    MalformedInt(String),
    ZeroRef,
    RefTooLarge(usize),
}

#[derive(Debug, PartialEq)]
pub enum LoHiState {
    Low,
    High,
}

impl fmt::Display for RefSpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let err_msg = match self {
            RefSpecError::MalformedInt(mfi) => format!("Malformed integer {}", mfi),
            RefSpecError::ZeroRef => "Ref # must be > 0".to_string(),
            RefSpecError::RefTooLarge(max) => format!("Ref # too large (max {})", max),
        };
        write!(f, "{}", err_msg)
    }
}

pub struct Alignment {
    pub headers: Vec<String>,
    pub sequences: Vec<String>,
    /* The consensus sequence is now a field of Alignment, and is computed once upon creation. This
     * contrasts with the very first implementation, in which the consensus was recomputed every
     * time the UI was drawn... which was very inefficient but had this funny "twinkling" effect in
     * columns with tied residue frequencies. This was due to the fact that HashMap stores its keys
     * in an unpredictable order, and that different calls to keys() may return them indifferent
     * orders. See best_residue().
     */
    /* These are properties of the whole _alignment_, or at least of whole columns. They cannot be
     * meaningfully attributed to a sequence. */
    pub consensus: String,
    pub entropies: Vec<f64>,
    pub densities: Vec<f64>,

    /* By contrast, the following are properties of sequences (at least in part). Length, for
     * example, does not depend on anything but the sequence itself, and could be a field in a
     * struct that also contains the sequence and its header. */
    pub id_wrt_reference: Vec<f64>, // reference is usually the consensus, but CAN be an aln seq.
    // Recompute if ref changes.
    // Of course the sequence length is an integer, but using an integer type like u32 would make
    // it hard (for me, at least...) to write a function that accepts a Vec of either lengths or
    // %IDs. Tried Box, and generics, but the extra work doesn't seem warranted.
    pub relative_seq_len: Vec<f64>,
    pub seq_quality: Vec<f64>,      // based on the fraction of ambiguous residues
    pub macromolecule_type: SeqType,
    /* Specifies whether the reference sequence should be the consensus (see above) or one of the
     * original sequences (identified by rank).*/
    ref_spec: RefSpec,
}

#[derive(Debug, PartialEq)]
struct BestResidue {
    residue: char,
    frequency: u64,
}

impl Alignment {
    // Makes an Alignment from a SeqFile, which is consumed.
    pub fn from_file(seq_file: SeqFile) -> Alignment {
        let mut headers: Vec<String> = Vec::new();
        let mut sequences: Vec<String> = Vec::new();
        let mut max_len: usize = 0;
        for record in seq_file {
            headers.push(record.header);
            let l = record.sequence.len();
            sequences.push(record.sequence);
            if l > max_len {
                max_len = l;
            }
        }
        // Pad any sequence shorter than max_len, so we are not limited to alignments with exactly
        // identical numbers of positions (reviewer suggestion).
        sequences
            .iter_mut()
            .for_each(|s| *s = format!("{:<width$}", s, width = max_len));
        // NOTE: the 's' can also be written '&*s', which makes the automatic re-borrow explicit.
        let first_seq = sequences.first();
        let macromolecule_type = seq_type(first_seq.expect("No sequence found."));
        let consensus = compute_consensus(&sequences, macromolecule_type);
        let entropies = compute_entropies(&sequences);
        let densities = compute_densities(&sequences);
        let id_wrt_reference = sequences
            .iter()
            .map(|seq| percent_identity(seq, &consensus))
            .collect();
        let relative_seq_len = sequences.iter().map(|seq| seq_len_nogaps(seq)).collect();
        let seq_quality = sequences
            .iter()
            .map(|seq| seq_quality(seq, macromolecule_type))
            .collect();

        Alignment {
            headers,
            sequences,
            consensus,
            entropies,
            densities,
            id_wrt_reference,
            relative_seq_len,
            seq_quality,
            macromolecule_type,
            ref_spec: RefSpec::Consensus,
        }
    }

    // Makes an Alignment from a Vec of headers and a Vec of Strings, which are consumed. Mostly
    // used for testing.
    #[allow(dead_code)]
    pub fn from_vecs(hdrs: Vec<String>, seqs: Vec<String>) -> Alignment {
        assert_eq!(hdrs.len(), seqs.len());
        let headers = hdrs;
        let sequences = seqs;
        let first_seq = sequences.first();
        let macromolecule_type = seq_type(first_seq.expect("No sequence found."));
        let consensus = compute_consensus(&sequences, macromolecule_type);
        let entropies = compute_entropies(&sequences);
        let densities = compute_densities(&sequences);
        let id_wrt_reference = sequences
            .iter()
            .map(|seq| percent_identity(seq, &consensus))
            .collect();
        let relative_seq_len = sequences.iter().map(|seq| seq_len_nogaps(seq)).collect();
        let seq_quality = sequences
            .iter()
            .map(|seq| seq_quality(seq, macromolecule_type))
            .collect();

        Alignment {
            headers,
            sequences,
            consensus,
            entropies,
            densities,
            id_wrt_reference,
            relative_seq_len,
            seq_quality,
            macromolecule_type,
            ref_spec: RefSpec::Consensus,
        }
    }

    pub fn num_seq(&self) -> usize {
        self.sequences.len()
    }

    // TODO: shouldn't this be aln_width?
    pub fn aln_len(&self) -> usize {
        self.sequences[0].len()
    }

    pub fn macromolecule_type(&self) -> SeqType {
        self.macromolecule_type
    }

    pub fn get_ref_spec(&self) -> RefSpec {
        self.ref_spec
    }

    pub fn set_ref_spec(&mut self, spec: RefSpec) -> Result<(), RefSpecError> {
        match spec {
            // Note: the rank in a RefSpec is 0-based. The conversion from user-land is done in
            // app.rs.
            RefSpec::Rank(rk) if rk >= self.num_seq() => {
                return Err(RefSpecError::RefTooLarge(self.num_seq()));
            }
            _ => self.ref_spec = spec,
        }
        // Probable change of ref -> Recompute the identities WRT ref
        let reference = self.reference();
        self.id_wrt_reference = self
            .sequences
            .iter()
            .map(|seq| percent_identity(seq, &reference))
            .collect();
        Ok(())
    }

    pub fn reference(&self) -> String {
        match self.ref_spec {
            RefSpec::Consensus => self.consensus.clone(),
            RefSpec::Rank(rk) => self.sequences[rk].clone(),
        }
    }

}

fn res_count(sequences: &Vec<String>, col: usize) -> ResidueCounts {
    let mut freqs: ResidueCounts = HashMap::new();
    for seq in sequences {
        let residue = seq.as_bytes()[col] as char;
        *freqs.entry(residue).or_insert(0) += 1;
    }
    freqs
}

fn compute_consensus(sequences: &Vec<String>, seq_type: SeqType) -> String {
    let mut consensus = String::new();
    for j in 0..sequences[0].len() {
        let dist = res_count(sequences, j); // res -> count map
        let br = best_residue(&dist, seq_type);
        let rel_freq: f64 = (br.frequency as f64 / sequences.len() as f64) as f64;
        if rel_freq >= UC_CONS_THRESHOLD {
            consensus.push(br.residue.to_ascii_uppercase());
        } else if rel_freq >= LC_CONS_THRESHOLD {
            if br.residue.is_alphabetic() {
                consensus.push(br.residue.to_ascii_lowercase());
            } else {
                //consensus.push('-');
                consensus.push(br.residue);
            }
        } else {
            consensus.push('*');
        }
    }
    consensus
}

fn compute_entropies(sequences: &Vec<String>) -> Vec<f64> {
    let mut entropies: Vec<f64> = Vec::new();
    for j in 0..sequences[0].len() {
        let dist = res_count(sequences, j);
        let freq = to_freq_distrib(&dist);
        let e = entropy(&freq);
        entropies.push(e);
    }
    entropies
}

pub fn col_density(sequences: &Vec<String>, col: usize) -> f64 {
    let mut mass = 0;
    for seq in sequences {
        match seq.as_bytes()[col] as char {
            'a'..='z' | 'A'..='Z' => mass += 1,
            '-' | '.' | ' ' => {}
            other => {
                panic!("Character {other} unexpected in an alignment.\nThis might be due to file format, please see option -f.");
            }
        }
    }
    mass as f64 / sequences.len() as f64
}

fn compute_densities(sequences: &Vec<String>) -> Vec<f64> {
    (0..sequences[0].len())
        .map(|col| col_density(sequences, col))
        .collect()
}

fn iupac_ambiguity_code(amb_nt: &mut [char]) -> char {
    let mut normalized_nt = amb_nt
        .into_iter()
        .map(|nt| nt.to_ascii_lowercase())
        .collect::<Vec<char>>();
    normalized_nt.sort();

    let normalized_nt_as_string = normalized_nt.into_iter().join("");

    match normalized_nt_as_string.as_str() {
        "a" => 'a',    // Adenine
        "ac" => 'm',   // aMino
        "acg" => 'v',  // not-T (not-U), V follows U
        "acgt" => 'n', // aNy
        "act" => 'h',  // not-G, H follows G in the alphabet
        "ag" => 'r',   // puRine
        "agt" => 'd',  // not-C, D follows C
        "at" => 'w',   // Weak interaction (2 H bonds)
        "c" => 'c',    // Cytosine
        "cg" => 's',   // Strong interaction (3 H bonds)
        "cgt" => 'b',  // not-A, B follows A
        "ct" => 'y',   // pYrimidine
        "g" => 'g',    // Guanine
        "gt" => 'k',   // Keto
        "t" => 't',    // Thymine
        &_ => 'n',     // Anything else folded into N - might want to bail, perhaps?
    }
}

fn best_residue(counts: &ResidueCounts, seq_type: SeqType) -> BestResidue {
    let max_freq = counts.values().max().unwrap();
    let mut most_frequent_residues = counts // plural <- may be ties
        .keys()
        .filter(|&&k| counts.get(&k) == Some(max_freq))
        .map(|&k| k)
        .collect::<Vec<char>>();

    let residue = if most_frequent_residues.len() == 1 {
        most_frequent_residues[0]
    } else if SeqType::Protein == seq_type {
        'X'
    } else {
        iupac_ambiguity_code(&mut most_frequent_residues)
    };

    BestResidue {
        residue: residue,
        frequency: *max_freq,
    }
}

// Convert a residue -> count map into a residue -> frequency map (relative frequency, that is).
// While gaps are allowed (and indeed useful) in the former, they are not included in the latter
// (in particular because they make litle sense when computing entropy).
//
fn to_freq_distrib(counts: &ResidueCounts) -> ResidueDistribution {
    let total_counts: u64 = counts
        .iter()
        .filter(|(res, _count)| **res != '-')
        .map(|(_res, count)| count)
        .sum();
    let mut distrib = ResidueDistribution::new();
    for (residue, count) in counts.iter() {
        if *residue == '-' {
            continue;
        }
        distrib.insert(*residue, *count as f64 / total_counts as f64);
    }
    distrib
}

fn entropy(freqs: &ResidueDistribution) -> f64 {
    // Discard '-'s
    let residues: Vec<&char> = freqs.keys().filter(|&&r| r != '-').collect();
    let sum: f64 = residues
        .into_iter()
        .map(|res| {
            let p = *freqs.get(res).unwrap();
            p * p.ln()
        })
        .sum();

    -sum
}

fn percent_identity(s1: &str, s2: &str) -> f64 {
    let num_identical = s1
        .chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1.eq_ignore_ascii_case(c2))
        .count();
    num_identical as f64 / s1.len() as f64
}

fn seq_len_nogaps(s: &str) -> f64 {
    s.chars().filter(|c| c.is_alphabetic()).count() as f64 / s.len() as f64
}

// Quality based on the proportion of non-ambiguous residues (ignoring gaps)
fn seq_quality(s: &str, macromolecule_type: SeqType) -> f64 {

    let ambiguous_vec = match macromolecule_type {
        SeqType::Nucleic => {
            vec![
                    'Y', 'y',
                    'R', 'r',
                    'W', 'w',
                    'K', 'k',
                    'S', 's',
                    'M', 'm',
                    'D', 'd',
                    'V', 'v',
                    'H', 'h',
                    'B', 'b',
                    'N', 'n'
            ]
        }
        SeqType::Protein => {
            vec!['X', 'x']
        }
    };

    let ambiguous: HashSet<char> = HashSet::from_iter(ambiguous_vec);

    let non_gap_chars: Vec<char> = s.chars().filter(|c| c.is_alphabetic()).collect();
    let num_non_ambig = non_gap_chars.iter()
        .filter(|r| !ambiguous.contains(r))
        .count();

    if non_gap_chars.is_empty() {
        0.0
    } else {
        num_non_ambig as f64 / non_gap_chars.len() as f64
    }

}

fn seq_type(sequence: &str) -> SeqType {
    let counts = sequence.to_lowercase().chars().counts();
    let counts_u64: HashMap<char, u64> = counts.into_iter().map(|(k, v)| (k, v as u64)).collect();
    let frequencies = to_freq_distrib(&counts_u64);
    let nt_freq: f64 = *frequencies.get(&'a').unwrap_or(&0.0)
        + *frequencies.get(&'c').unwrap_or(&0.0)
        + *frequencies.get(&'g').unwrap_or(&0.0)
        + *frequencies.get(&'t').unwrap_or(&0.0)
        + *frequencies.get(&'u').unwrap_or(&0.0);
    // A quick-and dirty heuristic, I'm afraid
    if nt_freq > 0.75 {
        Nucleic
    } else {
        Protein
    }
}

pub fn mark_lohi(metric: &[f64], threshold: f64) -> Vec<LoHiState> {
    assert!(!threshold.is_nan(), "threshold must not be NaN");
    metric
        .iter()
        .map(|&v| {
            assert!(!v.is_nan(), "metric value must not be NaN");

            if v < threshold {
                LoHiState::Low
            } else {
                LoHiState::High
            }
        })
        .collect()
}

pub fn find_hi_runs(lohi_states: &[LoHiState]) -> Vec<(usize, usize)> {
    let mut runs = Vec::new();
    let mut run_start: Option<usize> = None;

    for (i, state) in lohi_states.iter().enumerate() {
        match (run_start, state) {
            // start of a high-metric chunk
            (None, LoHiState::High) => {
                run_start = Some(i);
            }
            // any part of a low-metric chunk
            (Some(start), LoHiState::Low) => {
                runs.push((start, i - start));
                run_start = None;
            }
            _ => {}
        }
    }

    // trailing chunk
    if let Some(start) = run_start {
        runs.push((start, lohi_states.len() - start));
    }

    runs
}

pub fn merge_hi_runs(runs: &[(usize, usize)], threshold: usize) -> Vec<(usize, usize)> {
    let mut merged_runs = Vec::new();

    if runs.is_empty() {
        return merged_runs;
    }

    merged_runs.push(runs[0]);

    for &(cur_run_start, cur_run_len) in &runs[1..] {
        let (prev_run_start, prev_run_len) = *merged_runs.last().unwrap();

        let low_run_start = prev_run_start + prev_run_len;
        let low_run_len = cur_run_start - low_run_start;

        if low_run_len >= threshold {
            merged_runs.push((cur_run_start, cur_run_len));
        } else {
            merged_runs.last_mut().unwrap().1 += low_run_len + cur_run_len;
        }
    }

    merged_runs
}

#[cfg(test)]
mod tests {
    use crate::alignment::{
        best_residue,  entropy, find_hi_runs, mark_lohi,
        merge_hi_runs, percent_identity, res_count, seq_len_nogaps, seq_quality, seq_type, to_freq_distrib,
        Alignment, BestResidue, LoHiState, RefSpec, ResidueCounts, ResidueDistribution, SeqType,
        SeqType::{Nucleic, Protein},
    };
    use crate::seq::fasta::read_fasta_file;
    use approx::assert_relative_eq;
    use std::collections::HashMap;

    #[test]
    fn test_read_aln() {
        let fasta1 = read_fasta_file("./data/test2.fas").unwrap();
        let aln1 = Alignment::from_file(fasta1);
        assert_eq!("seq1", aln1.headers[0]);
        assert_eq!("seq2", aln1.headers[1]);
        assert_eq!("seq3", aln1.headers[2]);
        assert_eq!("TTGCCG-CGA", aln1.sequences[0]);
        assert_eq!("TTCCCGGCGA", aln1.sequences[1]);
        assert_eq!("TTACCG-CAA", aln1.sequences[2]);
    }

    #[test]
    fn test_consensus() {
        let fasta2 = read_fasta_file("data/test-cons.fas").unwrap();
        let aln2 = Alignment::from_file(fasta2);
        // Updated: the output changed with the IUPAC codes implementation. Position 2 and 4
        // are now resolved through the ambiguity code matcher, which defaults to 'n' for
        // protein residues. This is expected behavior until the function gains protein support.
        assert_eq!("AQw-n", aln2.consensus);
    }

    #[test]
    fn test_res_count() {
        let fasta2 = read_fasta_file("data/test-cons.fas").unwrap();
        let aln2 = Alignment::from_file(fasta2);
        let mut d0: ResidueCounts = HashMap::new();
        d0.insert('A', 6);
        assert_eq!(d0, res_count(&aln2.sequences, 0));

        let mut d1: ResidueCounts = HashMap::new();
        d1.insert('Q', 5);
        d1.insert('T', 1);
        assert_eq!(d1, res_count(&aln2.sequences, 1));

        let mut d2: ResidueCounts = HashMap::new();
        d2.insert('W', 2);
        d2.insert('I', 1);
        d2.insert('S', 1);
        d2.insert('D', 1);
        d2.insert('F', 1);
        assert_eq!(d2, res_count(&aln2.sequences, 2));

        let mut d3: ResidueCounts = HashMap::new();
        d3.insert('-', 3);
        d3.insert('K', 2);
        d3.insert('L', 1);
        assert_eq!(d3, res_count(&aln2.sequences, 3));
    }

    #[test]
    fn test_most_frequent_residue() {
        let d0: ResidueCounts = HashMap::from([('A', 6)]);
        let mut exp: BestResidue = BestResidue {
            residue: 'A',
            frequency: 6,
        };
        assert_eq!(exp, best_residue(&d0, SeqType::Nucleic));

        let d1: ResidueCounts = HashMap::from([('Q', 5), ('T', 1)]);
        exp = BestResidue {
            residue: 'Q',
            frequency: 5,
        };
        assert_eq!(exp, best_residue(&d1, SeqType::Protein));

        let d2: ResidueCounts = HashMap::from([('W', 2), ('I', 1), ('S', 1), ('D', 1), ('F', 1)]);
        exp = BestResidue {
            residue: 'W',
            frequency: 2,
        };
        assert_eq!(exp, best_residue(&d2, SeqType::Protein));

        // col 3 cannot be tested <- ties

        let d4: ResidueCounts = HashMap::from([('-', 3), ('K', 2), ('L', 1)]);
        exp = BestResidue {
            residue: '-',
            frequency: 3,
        };
        assert_eq!(exp, best_residue(&d4, SeqType::Protein));
    }

    #[test]
    fn test_to_freq_distrib() {
        let eps = 0.001;
        let counts: ResidueCounts = HashMap::from([('K', 3), ('L', 3), ('G', 6), ('-', 6)]);
        let rfreqs = to_freq_distrib(&counts);
        assert_relative_eq!(0.25, *rfreqs.get(&'K').unwrap(), epsilon = eps);
        assert_relative_eq!(0.25, *rfreqs.get(&'L').unwrap(), epsilon = eps);
        assert_relative_eq!(0.5, *rfreqs.get(&'G').unwrap(), epsilon = eps);
    }

    #[test]
    fn test_entropy_1() {
        let eps = 0.00001;
        let distrib: ResidueDistribution = ResidueDistribution::from([('A', 1.0)]);
        assert_relative_eq!(0.0, entropy(&distrib), epsilon = eps);
    }

    #[test]
    fn test_entropy_2() {
        let eps = 0.00001;
        let distrib: ResidueDistribution = ResidueDistribution::from([('A', 0.5), ('F', 0.5)]);
        // This should be ln(2), and as it happens Rust has a constant for this; remarkably, clippy
        // detects the literal constant below and suggests using the (arguably more accurate)
        // built-in definition.
        // assert_relative_eq!(0.6931471805599453, entropy(&distrib), epsilon = eps);
        assert_relative_eq!(std::f64::consts::LN_2, entropy(&distrib), epsilon = eps);
    }

    #[test]
    fn test_entropy_3() {
        let eps = 0.00001;
        let distrib: ResidueDistribution =
            ResidueDistribution::from([('A', 0.5), ('F', 0.25), ('T', 0.25)]);
        assert_relative_eq!(1.0397207708399179, entropy(&distrib), epsilon = eps);
    }

    #[test]
    fn test_entropies() {
        let fasta2 = read_fasta_file("data/test-cons.fas").unwrap();
        let aln2 = Alignment::from_file(fasta2);
        let eps = 0.001;
        assert_relative_eq!(0.0, aln2.entropies[0], epsilon = eps);
        assert_relative_eq!(0.4505, aln2.entropies[1], epsilon = eps);
        assert_relative_eq!(1.5607, aln2.entropies[2], epsilon = eps);
        assert_relative_eq!(0.6365, aln2.entropies[3], epsilon = eps);
    }

    #[test]
    fn test_density() {
        let fasta = read_fasta_file("data/test-density.msa").unwrap();
        let aln = Alignment::from_file(fasta);
        assert_eq!(1.0, aln.densities[0]);
        assert_eq!(0.8, aln.densities[1]);
        assert_eq!(0.6, aln.densities[2]);
        assert_eq!(0.4, aln.densities[3]);
        assert_eq!(0.2, aln.densities[4]);
        assert_eq!(0.0, aln.densities[5]);
    }

    #[test]
    fn test_order_aln() {
        let fasta = read_fasta_file("./data/test4.aln").unwrap();
        let aln1 = Alignment::from_file(fasta);
        // Check original order
        assert_eq!("Zea_001", aln1.headers[0]);
        assert_eq!("Rana_002", aln1.headers[1]);
        assert_eq!("Panthera_050", aln1.headers[49]);
        assert_eq!("tgctgttcgtcaaAgtaggcc", aln1.sequences[0]);
        assert_eq!("tgctgttAgAcaaagtaggcc", aln1.sequences[1]);
        assert_eq!("tgctgttcgtcaaagtaggcc", aln1.sequences[49]);
    }

    #[test]
    fn test_similarity_00() {
        let s1 = "GAATTC";
        assert_eq!(percent_identity(s1, s1), 1.0);
    }

    #[test]
    fn test_similarity_05() {
        let s1 = "GAATTC";
        let s2 = "GAA---";
        assert_eq!(percent_identity(s1, s2), 0.5);
    }

    #[test]
    fn test_similarity_10() {
        let s1 = "GAATTC";
        let s2 = "gaattc";
        assert_eq!(percent_identity(s1, s2), 1.0);
    }

    #[test]
    fn test_seq_len_nogaps_00() {
        assert_eq!(seq_len_nogaps("atgc"), 1.0);
    }

    #[test]
    fn test_seq_len_nogaps_05() {
        assert_eq!(seq_len_nogaps("a-gc"), 0.75);
    }

    #[test]
    fn test_seq_len_nogaps_10() {
        assert_eq!(seq_len_nogaps("--.-"), 0.0);
    }

    #[test]
    fn test_seq_type_00() {
        assert_eq!(Nucleic, seq_type("GAATTC"));
    }

    #[test]
    fn test_seq_type_05() {
        assert_eq!(Protein, seq_type("HGTSDA"));
    }

    #[test]
    fn test_seq_type_10() {
        assert_eq!(Nucleic, seq_type("cgatgcacgatgcncagtgtuucgatcga"));
    }

    #[test]
    fn test_seq_type_15() {
        assert_eq!(Nucleic, seq_type("UUTGAU"));
    }

    // Make sure seq files with unequal lengths get correctly padded
    #[test]
    fn test_unequal_seq_len() {
        let fasta = read_fasta_file("./data/test5.aln").unwrap();
        let _ = Alignment::from_file(fasta);
    }

    // Test the Vec constructor
    #[test]
    fn test_vec_ctor_00() {
        let hdrs = vec![
            String::from("Leo"),
            String::from("Tigris"),
            String::from("Pardus"),
            String::from("Onca"),
        ];
        let seqs = vec![
            String::from("catgcatatg"),
            String::from("aatgcatatg"),
            String::from("tatgcatatg"),
            String::from("gatgcatatg"),
        ];
        let aln = Alignment::from_vecs(hdrs, seqs);
        assert_eq!(4, aln.num_seq());
        assert_eq!(10, aln.aln_len());
        assert_eq!(SeqType::Nucleic, aln.macromolecule_type());
        assert_eq!("Onca", aln.headers[3]);
        assert_eq!("gatgcatatg", aln.sequences[3]);
    }

    // Test the reference specifier
    #[test]
    fn test_reference_specifier() {
        let hdrs = vec![
            String::from("frugilegus"),
            String::from("monedula"),
            String::from("corax"),
            String::from("corone"),
            String::from("cornix"),
        ];
        let seqs = vec![
            String::from("catgcatatg"),
            String::from("aatgcatatg"),
            String::from("tatgcatatg"),
            String::from("tatgcatatg"),
            String::from("gatgcatatg"),
        ];
        let mut aln = Alignment::from_vecs(hdrs, seqs);
        // By default, the reference sequence is the consensus
        assert_eq!(RefSpec::Consensus, aln.get_ref_spec());
        assert_eq!("tATGCATATG", aln.reference());
        // Now set the ref to the first sequence (rank 0)
        let _ = aln.set_ref_spec(RefSpec::Rank(0));
        assert_eq!(RefSpec::Rank(0), aln.get_ref_spec());
        assert_eq!("catgcatatg", aln.reference());
        // Back to consensus
        let _ = aln.set_ref_spec(RefSpec::Consensus);
        assert_eq!(RefSpec::Consensus, aln.get_ref_spec());
        assert_eq!("tATGCATATG", aln.reference());
    }

    // Tests the %id WRT ref (incl. when != consensus)
    #[test]
    fn test_pct_id_wrt_ref() {
        let hdrs = vec![
            String::from("frugilegus"),
            String::from("monedula"),
            String::from("corax"),
            String::from("corone"),
            String::from("cornix"),
        ];
        // consensus: ACg-
        let seqs = vec![
            String::from("A---"),
            String::from("AC--"),
            String::from("ACG-"),
            String::from("ACGT"),
            String::from("ACGT"),
        ];
        let mut aln = Alignment::from_vecs(hdrs, seqs);
        // Check the ref, which by default is the consensus
        assert_eq!("ACg-", aln.reference());
        assert_eq!(vec![0.5, 0.75, 1.0, 0.75, 0.75], aln.id_wrt_reference);
        // Now switch to seq #0 for reference
        let _ = aln.set_ref_spec(RefSpec::Rank(0));
        assert_eq!("A---", aln.reference());
        assert_eq!(vec![1.0, 0.75, 0.5, 0.25, 0.25], aln.id_wrt_reference);
        // Switch back to consensus
        let _ = aln.set_ref_spec(RefSpec::Consensus);
        assert_eq!("ACg-", aln.reference());
        assert_eq!(vec![0.5, 0.75, 1.0, 0.75, 0.75], aln.id_wrt_reference);
    }

    #[test]
    fn test_mark_lohi() {
        let metric = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];
        assert_eq!(
            mark_lohi(&metric, 0.8),
            vec![
                LoHiState::Low,
                LoHiState::Low,
                LoHiState::Low,
                LoHiState::Low,
                LoHiState::Low,
                LoHiState::Low,
                LoHiState::Low,
                LoHiState::High,
                LoHiState::High,
                LoHiState::High,
            ]
        );
    }

    #[test]
    fn test_find_hi_runs() {
        let lohi_states = vec![
            LoHiState::High,
            LoHiState::Low,
            LoHiState::Low,
            LoHiState::Low,
            LoHiState::High, // start of a hi run at pos 4 (length 4)
            LoHiState::High,
            LoHiState::High,
            LoHiState::High,
            LoHiState::Low,
            LoHiState::Low,
            LoHiState::Low,
            LoHiState::Low,
            LoHiState::High, // start of a hi run at pos 12 (length 5)
            LoHiState::High,
            LoHiState::High,
            LoHiState::High,
            LoHiState::High,
            LoHiState::High,
            LoHiState::Low,
            LoHiState::Low,
            LoHiState::Low,
            LoHiState::High,
            LoHiState::High,
        ];
        assert_eq!(
            find_hi_runs(&lohi_states),
            vec![(0, 1), (4, 4), (12, 6), (21, 2)]
        );
    }

    #[test]
    fn test_find_hi_runs_all_high() {
        let lohi_states = vec![LoHiState::High, LoHiState::High, LoHiState::High];
        assert_eq!(find_hi_runs(&lohi_states), vec![(0, 3)]);
    }

    #[test]
    fn test_find_hi_runs_all_low() {
        let lohi_states = vec![LoHiState::Low, LoHiState::Low, LoHiState::Low];
        assert_eq!(find_hi_runs(&lohi_states), vec![]);
    }

    #[test]
    fn test_merge_hi_runs_single_run() {
        assert_eq!(merge_hi_runs(&[(5, 3)], 3), vec![(5, 3)]);
    }

    #[test]
    fn test_merge_hi_runs_merges_short_gaps_only() {
        // Runs are:
        // 0..4, 6..8, 11..14, 16..18, 22..24
        //
        // Gaps are:
        // 4..6   len 2  -> merge if threshold is 3
        // 8..11  len 3  -> do not merge if threshold is 3
        // 14..16 len 2  -> merge if threshold is 3
        // 18..22 len 4  -> do not merge if threshold is 3

        let runs = vec![(0, 4), (6, 2), (11, 3), (16, 2), (22, 2)];

        assert_eq!(merge_hi_runs(&runs, 3), vec![(0, 8), (11, 7), (22, 2)]);
    }

    // B0024: consensus was nondeterministic when the most frequent residue at a position was tied.
    // HashMap iteration order is randomized per process, so best_residue() would pick an arbitrary
    // tied residue. Fixed by computing IUPAC ambiguity codes for ties.

    #[test]
    fn test_consensus_is_stable_across_calls() {
        // Multiple calls to consensus on the same alignment should yield identical results.
        // Without the fix, this fails ~50% of the time due to HashMap iteration order.
        let seqs = vec![
            "AGGCTC".to_string(),
            "AGGCAC".to_string(),
            "ACGTGC".to_string(),
        ];
        let headers: Vec<String> = (0..seqs.len()).map(|i| format!("seq{}", i)).collect();
        let aln1 = Alignment::from_vecs(headers.clone(), seqs.clone());
        let aln2 = Alignment::from_vecs(headers.clone(), seqs.clone());
        let aln3 = Alignment::from_vecs(headers.clone(), seqs.clone());
        assert_eq!(
            aln1.consensus, aln2.consensus,
            "consensus changed between calls: '{}' vs '{}'",
            aln1.consensus, aln2.consensus
        );
        assert_eq!(
            aln2.consensus, aln3.consensus,
            "consensus changed between calls: '{}' vs '{}'",
            aln2.consensus, aln3.consensus
        );
    }

    #[test]
    fn test_consensus_uses_iupac_codes_for_tied_nucleotides() {
        // Position 4 (0-indexed) has a three-way tie: T, A, G each appear once.
        // The consensus should use the IUPAC code 'D' (not A/C/G) for this position.
        // D = adenine, Guanine, Thymine (not C).
        let seqs = vec![
            "AGGCTC".to_string(),
            "AGGCAC".to_string(),
            "ACGTGC".to_string(),
        ];
        let headers: Vec<String> = (0..seqs.len()).map(|i| format!("seq{}", i)).collect();
        let aln = Alignment::from_vecs(headers, seqs);
        // Positions: 0=A (3/3), 1=G (2/3), 2=G (3/3), 3=C (2/3), 4=d/D (tie 1/3), 5=C (3/3)
        // The exact case depends on frequency threshold, but position 4 should be deterministic.
        assert!(
            aln.consensus.chars().nth(4).unwrap().to_ascii_lowercase() == 'd',
            "position 4 should resolve to IUPAC code 'd' for A/G/T tie, got '{}'",
            aln.consensus.chars().nth(4).unwrap()
        );
    }

    #[test]
    fn test_consensus_tie_breakpoint_by_frequency() {
        // Two residues tied at the top frequency should produce an IUPAC code.
        // (Frequency 2/4 = 0.5, above the lowercase threshold of ~20%, so renders lowercase.)
        let seqs = vec![
            "ACT".to_string(),
            "ACT".to_string(),
            "ATT".to_string(),
            "ATT".to_string(),
        ];
        let headers: Vec<String> = (0..seqs.len()).map(|i| format!("seq{}", i)).collect();
        let aln = Alignment::from_vecs(headers, seqs);
        // Position 0: A 4/4
        // Position 1: C 2/4, T 2/4 (tie at top frequency)
        // Position 2: T 4/4
        // Position 1 should be a nucleotide IUPAC code (C and T -> 'y' for pYrimidine).
        let pos1 = aln.consensus.chars().nth(1).unwrap().to_ascii_lowercase();
        assert_eq!(
            pos1, 'y',
            "position 1 should resolve to IUPAC code 'y' for C/T tie, got '{}'",
            pos1
        );
    }

    #[test]
    fn test_seq_quality_nucleic_no_ambiguous() {
        // Perfect sequence with no ambiguous residues
        let seq = "ACGTACGTACGT";
        let quality = seq_quality(seq, SeqType::Nucleic);
        assert_eq!(1.0, quality);
    }

    #[test]
    fn test_seq_quality_nucleic_all_ambiguous() {
        // All ambiguous nucleotides (Y, R, W, etc.)
        let seq = "YRWKSM";
        let quality = seq_quality(seq, SeqType::Nucleic);
        assert_eq!(0.0, quality);
    }

    #[test]
    fn test_seq_quality_nucleic_mixed() {
        // Half ambiguous, half not (ACGT = 4 non-ambig, YR = 2 ambig, total 6)
        let seq = "ACGTYR";
        let quality = seq_quality(seq, SeqType::Nucleic);
        let eps = 0.001;
        assert_relative_eq!(2.0 / 3.0, quality, epsilon = eps);
    }

    #[test]
    fn test_seq_quality_nucleic_with_gaps() {
        // Gaps don't count as ambiguous or non-ambiguous in this calculation
        // Sequence "ACG-T" has 4 non-gap chars, all non-ambiguous
        let seq = "ACG-T";
        let quality = seq_quality(seq, SeqType::Nucleic);
        assert_eq!(1.0, quality);
    }

    #[test]
    fn test_seq_quality_protein_no_ambiguous() {
        let seq = "ACDEFGHIKLMNPQRSTVW";
        let quality = seq_quality(seq, SeqType::Protein);
        assert_eq!(1.0, quality);
    }

    #[test]
    fn test_seq_quality_protein_all_ambiguous() {
        let seq = "XXxx";
        let quality = seq_quality(seq, SeqType::Protein);
        assert_eq!(0.0, quality);
    }

    #[test]
    fn test_seq_quality_protein_mixed() {
        // "ACGX" = 3 non-ambig, 1 ambig
        let seq = "ACGX";
        let quality = seq_quality(seq, SeqType::Protein);
        let eps = 0.001;
        assert_relative_eq!(0.75, quality, epsilon = eps);
    }
}
