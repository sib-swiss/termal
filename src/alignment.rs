// NOTE the gotcha: declaring the module in this file (only) results in an unresolved import.
// Declaring it in main.rs, i.e. at the crate root, makes it "visible" to the whole crate.
//
//mod vec_f64_aux;
use std::collections::HashMap;

use rasta::FastaFile;

type ResidueDistribution = HashMap<char, f64>;
type ResidueCounts = HashMap<char, u64>;

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
    pub consensus: String,
    pub entropies: Vec<f64>,
    pub densities: Vec<f64>,
}

#[derive(Debug, PartialEq)]
struct BestResidue {
    residue: char,
    frequency: u64,
}

impl Alignment {
    // Makes an Alignment from a FastaFile, which is consumed.
    pub fn new(fasta: FastaFile) -> Alignment {
        let mut headers: Vec<String> = Vec::new();
        let mut sequences: Vec<String> = Vec::new();
        for record in fasta {
            headers.push(record.header);
            sequences.push(record.sequence);

        }
        let consensus = consensus(&sequences);
        let entropies = entropies(&sequences);
        let densities = densities(&sequences);

        Alignment {
            headers,
            sequences,
            consensus,
            entropies,
            densities,
        }
    }

    pub fn num_seq(&self) -> usize { self.sequences.len() }

    pub fn aln_len(&self) -> usize { self.sequences[0].len() }


}

fn res_count(sequences: &Vec<String>, col: usize) ->  ResidueCounts {
    let mut freqs: ResidueCounts = HashMap::new();
    for seq in sequences {
        let residue = seq.as_bytes()[col] as char;
        *freqs.entry(residue).or_insert(0) += 1;
    }
    freqs
}

pub fn consensus(sequences: &Vec<String>) -> String {
    let mut consensus = String::new();
    for j in 0 .. sequences[0].len() {
        let dist = res_count(sequences, j);
        let br = best_residue(&dist);
        let rel_freq: f64 = (br.frequency as f64 / sequences.len() as f64) as f64;
        if rel_freq >= 0.8 {
            consensus.push(br.residue);
        } else if rel_freq >= 0.2 {
            if br.residue.is_alphabetic() {
                consensus.push((br.residue as u8 + 97 - 65) as char);
            } else {
                consensus.push('-'); 
            }
        } else {
            consensus.push('.');
        }
    }
    consensus
}

pub fn entropies(sequences: &Vec<String>) -> Vec<f64> {
    let mut entropies: Vec<f64> = Vec::new();
    for j in 0 .. sequences[0].len() {
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
            'a' ..= 'z' | 'A' ..= 'Z' => { mass += 1 }
            '-' => {}
            other => { panic!("Character {other} unexpected in an alignment."); }
        }
    }
    let ρ = mass as f64 / sequences.len() as f64;
    ρ
}

pub fn densities(sequences: &Vec<String>) -> Vec<f64> {
    (0 .. sequences[0].len()).map(|col| {
        col_density(sequences, col)
    }).collect()
}

fn best_residue(dist: &ResidueCounts) -> BestResidue {
    let max_freq = dist.values().max().unwrap();
    let most_frequent_residue = dist.keys()
        .find(|&&k| dist.get(&k) == Some(max_freq))
        .unwrap();

    BestResidue {
        residue: *most_frequent_residue,
        frequency: *max_freq,
    }
}

// Convert a residue -> count map into a residue -> frequency map (relative frequency, that is).
// While gaps are allowed (and indeed useful) in the former, they are not included in the latter
// (in particular because they make litle sense when computing entropy).
//
fn to_freq_distrib(counts: &ResidueCounts) -> ResidueDistribution {
    let total_counts: u64 = counts.iter()
        .filter(|(res, _count)| **res != '-')
        .map(|(_res, count)| count)
        .sum();
    let mut distrib = ResidueDistribution::new();
    for (residue, count) in counts.iter() {
        if *residue == '-' { continue; }
        distrib.insert(*residue, *count as f64 / total_counts as f64 );
    }
    distrib
}

fn entropy(freqs: &ResidueDistribution) -> f64 {
    // Discard '-'s
    let residues: Vec<&char> = freqs.keys().filter(|&&r| r != '-').collect();
    let sum: f64 = residues.into_iter().map(|res| {
        let p = *freqs.get(res).unwrap();
        p * p.ln()
    })
    .sum();
    -1.0 * sum
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rasta::read_fasta_file;
    use crate::alignment::{
        Alignment, BestResidue, best_residue,
            consensus, densities, entropies, entropy, ResidueCounts,
            ResidueDistribution, res_count, to_freq_distrib,
    };
    use approx::assert_relative_eq;

    #[test]
    fn test_read_aln() {
        let fasta1 = read_fasta_file("./data/test2.fas").unwrap();
        let aln1 = Alignment::new(fasta1);
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
        let aln2 = Alignment::new(fasta2);
        assert_eq!("AQw-", consensus(&aln2.sequences));
    }

    #[test]
    fn test_res_count() {
        let fasta2 = read_fasta_file("data/test-cons.fas").unwrap();
        let aln2 = Alignment::new(fasta2);
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
        let mut exp: BestResidue = BestResidue { residue: 'A', frequency: 6 };
        assert_eq!(exp, best_residue(&d0));

        let d1: ResidueCounts = HashMap::from([
        	('Q', 5),
        	('T', 1),
        ]);
        exp = BestResidue { residue: 'Q', frequency: 5 };
        assert_eq!(exp, best_residue(&d1));

        let d2: ResidueCounts = HashMap::from([
        	('W', 2),
        	('I', 1), 
        	('S', 1), 
        	('D', 1), 
        	('F', 1), 
        ]);
        exp = BestResidue { residue: 'W', frequency: 2 };
        assert_eq!(exp, best_residue(&d2));

        // col 3 cannot be tested <- ties

        let d4: ResidueCounts = HashMap::from([
        	('-', 3),
        	('K', 2),
        	('L', 1),
        ]);
        exp = BestResidue { residue: '-', frequency: 3 };
        assert_eq!(exp, best_residue(&d4));
    }

    #[test]
    fn test_to_freq_distrib() {
        let ε = 0.001;
        let counts: ResidueCounts = HashMap::from([
            ('K', 3),
            ('L', 3),
            ('G', 6),
            ('-', 6),
        ]);
        let rfreqs = to_freq_distrib(&counts);
        assert_relative_eq!(0.25, *rfreqs.get(&'K').unwrap(), epsilon = ε);
        assert_relative_eq!(0.25, *rfreqs.get(&'L').unwrap(), epsilon = ε);
        assert_relative_eq!(0.5, *rfreqs.get(&'G').unwrap(), epsilon = ε);
    }

    #[test]
    fn test_entropy_1() {
        let ε = 0.00001;
        let distrib: ResidueDistribution = ResidueDistribution::from([('A', 1.0)]);
        assert_relative_eq!(0.0, entropy(&distrib), epsilon = ε);
    }

    #[test]
    fn test_entropy_2() {
        let ε = 0.00001;
        let distrib: ResidueDistribution = ResidueDistribution::from([
            ('A', 0.5), ('F', 0.5),
        ]);
        assert_relative_eq!(0.6931471805599453, entropy(&distrib), epsilon = ε);
    }

    #[test]
    fn test_entropy_3() {
        let ε = 0.00001;
        let distrib: ResidueDistribution = ResidueDistribution::from([
            ('A', 0.5), ('F', 0.25), ('T', 0.25)
        ]);
        assert_relative_eq!(1.0397207708399179, entropy(&distrib), epsilon = ε);
    }

    #[test]
    fn test_entropies() {
        let fasta2 = read_fasta_file("data/test-cons.fas").unwrap();
        let aln2 = Alignment::new(fasta2);
        let entrs = entropies(&aln2.sequences);
        let ε = 0.001;
        assert_relative_eq!(0.0,    entrs[0], epsilon = ε);
        assert_relative_eq!(0.4505, entrs[1], epsilon = ε);
        assert_relative_eq!(1.5607, entrs[2], epsilon = ε);
        assert_relative_eq!(0.6365, entrs[3], epsilon = ε);
    }

    #[test]
    fn test_density() {
        let fasta = read_fasta_file("data/test-density.msa").unwrap();
        let aln = Alignment::new(fasta);
        let dens = densities(&aln.sequences);
        assert_eq!(1.0, dens[0]);
        assert_eq!(0.8, dens[1]);
        assert_eq!(0.6, dens[2]);
        assert_eq!(0.4, dens[3]);
        assert_eq!(0.2, dens[4]);
        assert_eq!(0.0, dens[5]);
    }

}
