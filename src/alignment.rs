use rasta::FastaFile;

struct Alignment {
    headers: Vec<String>,
    sequences: Vec<String>,
}

impl Alignment {
    pub fn new(fasta: FastaFile) -> Alignment {
        Alignment {
            headers: Vec::new(),
            sequences: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_read_aln() {
    }
}
