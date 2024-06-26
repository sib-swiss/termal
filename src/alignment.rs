use std::collections::HashMap;

use rasta::FastaFile;

pub struct Alignment {
    pub headers: Vec<String>,
    pub sequences: Vec<String>,
}

impl Alignment {
    // Makes an Alignment from a FastaFile, which is consumed.
    pub fn new(fasta: FastaFile) -> Alignment {
        let mut aln = Alignment {
            headers: Vec::new(),
            sequences: Vec::new(),
        };
        for record in fasta {
            aln.headers.push(record.header);
            aln.sequences.push(record.sequence);
        }
        aln
    }

    pub fn num_seq(&self) -> usize { self.sequences.len() }

    pub fn aln_len(&self) -> usize { self.sequences[0].len() }

    fn res_freq(&self, col: usize) -> HashMap<char, u64> {
        let mut freqs: HashMap<char, u64> = HashMap::new();
        for seq in &self.sequences {
            let residue = seq.as_bytes()[col] as char;
            *freqs.entry(residue).or_insert(0) += 1;
        }
        freqs
    }

    pub fn consensus(&self) -> String {
       todo!(); 
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use rasta::read_fasta_file;
    use crate::alignment::Alignment;

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
        assert_eq!("AQw.", aln2.consensus());
    }

    #[test]
    fn test_res_freq() {
        let fasta2 = read_fasta_file("data/test-cons.fas").unwrap();
        let aln2 = Alignment::new(fasta2);
        let mut d0: HashMap<char, u64> = HashMap::new();
        d0.insert('A', 6);
        assert_eq!(d0, aln2.res_freq(0));

        let mut d1: HashMap<char, u64> = HashMap::new();
        d1.insert('Q', 5);
        d1.insert('T', 1);
        assert_eq!(d1, aln2.res_freq(1));

        let mut d2: HashMap<char, u64> = HashMap::new();
        d2.insert('W', 2);
        d2.insert('I', 1); 
        d2.insert('S', 1); 
        d2.insert('D', 1); 
        d2.insert('F', 1); 
        assert_eq!(d2, aln2.res_freq(2));

        let mut d3: HashMap<char, u64> = HashMap::new();
        d3.insert('T', 2);
        d3.insert('G', 2);
        d3.insert('H', 2);
        assert_eq!(d3, aln2.res_freq(3));

        let mut d4: HashMap<char, u64> = HashMap::new();
        d4.insert('-', 3);
        d4.insert('K', 2);
        d4.insert('L', 1);
        assert_eq!(d4, aln2.res_freq(4));

    }
}
