use std::fs::File;
use std::path::Path;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct FastaRecord {
    pub header: String,
    pub sequence: String,
}

pub type FastaFile = Vec<FastaRecord>;

pub fn read_fasta_file<P: AsRef<Path>>(path: P) -> Result<FastaFile, std::io::Error> {
    let file = File::open(path)?;
    let mut result: FastaFile = Vec::new();
    let mut current_record = FastaRecord { header: String::new(), sequence: String::new() };
    let mut first_header = true;

    for line in BufReader::new(file).lines() {
        let l: String = line.unwrap();
        if l.starts_with(">") { 
            if first_header {
                first_header = false;
            } else {
                // push existing record
                result.push(current_record);
            }
            current_record = FastaRecord { header: String::new(), sequence: String::new() };
            current_record.header.push_str(&l[1..]);
        } else {
            // append line to current record'd sequence
            current_record.sequence.push_str(&l);
        }
    }
    result.push(current_record);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_fasta_file_1() {
        let path = "data/test1.fas";
        let fasta: FastaFile = read_fasta_file(path).expect("Test file not found");
        assert_eq!(fasta[0].header, "seq1");
        assert_eq!(fasta[0].sequence, "GAATTC");
    }

    #[test]
    fn test_read_fasta_file_2() {
        let path = "data/test2.fas";
        let fasta: FastaFile = read_fasta_file(path).expect("Test file not found");
        assert_eq!(fasta[0].header, "seq1");
        assert_eq!(fasta[0].sequence, "TTGCCG-CGA");
        assert_eq!(fasta[1].header, "seq2");
        assert_eq!(fasta[1].sequence, "TTCCCGGCGA");
        assert_eq!(fasta[2].header, "seq3");
        assert_eq!(fasta[2].sequence, "TTACCG-CAA");
    }

    #[test]
    fn test_read_fasta_file_3() {
        let path = "data/test3.pep";
        let fasta: FastaFile = read_fasta_file(path).expect("Test file not found");
        assert_eq!(fasta[0].header, "Some larger FastA record, with several lines");
        assert_eq!(fasta[0].sequence, "HWYQYDSWSWHQIQDPWVASLMTGSEHNTTIVDLNVLGAMDCLWLCYCQPECFEVFSLCIEVDLPSCCWAKALCAFHMWDSMAKQCWMPEMGEVSYFYALSMFHYFLLHSRPIQPWQTHHIPYDSIVVDLIANYFYNMIVQDVDKNSNIRFDRSVMRDVMIYEFENTYATGVVFNVNGKCGQFCKNMIYVGTIETQKEYEMFKNLDCAVQKRHNLQPNCENIAMKMRIQYNGKRFRMDYWERYRCNDIKQVLPQPFTEVAMEHRTFKLWPTTRLMMSNPKCRQCLEWAAVETGWIFTTNF");
    }
}

