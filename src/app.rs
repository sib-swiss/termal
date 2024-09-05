use rasta::read_fasta_file;

use crate::alignment::Alignment;

pub struct App {
    pub filename: String,
    pub alignment: Alignment,
}

impl App {
    pub fn new(path: &str) -> Result<App, std::io::Error> {
        let fasta_file = read_fasta_file(path)?;
        Ok(App {
            filename: path.to_string(),
            alignment: Alignment::new(fasta_file),
        })
    }

    // Computed properties (TODO: could be set in a struct member, as they do not change)

    pub fn num_seq(&self) -> u16 { self.alignment.num_seq().try_into().unwrap() }

    pub fn aln_len(&self) -> u16 { self.alignment.aln_len().try_into().unwrap() }

    pub fn output_info(&self) {
        println!("name: {}", self.filename);
        println!("nb_sequences: {}", self.num_seq());
        println!("nb_columns: {}", self.aln_len());
    }
}
