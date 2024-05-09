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
}

#[cfg(test)]
mod tests {
    use rasta::read_fasta_file;
    use crate::alignment::Alignment;

    #[test]
    fn test_read_aln() {
        let fasta1 = read_fasta_file("./data/test2.fas").unwrap();
        let aln1 = Alignment::new(fasta1);
        assert_eq!("seq1", aln1.headers[0]);
        assert_eq!("seq2", aln1.headers[1]);
        assert_eq!("seq3", aln1.headers[2]);
        assert_eq!("GAATTC", aln1.sequences[0]);
        assert_eq!("TTGCCGGCAA", aln1.sequences[1]);
        assert_eq!("TATAAT", aln1.sequences[2]);
    }
}
