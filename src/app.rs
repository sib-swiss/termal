use rasta::read_fasta_file;

use crate::alignment::Alignment;


pub struct App {
    pub filename: String,
    pub alignment: Alignment,
    pub top_line: u16,
    pub leftmost_col: u16,
}

impl App {
    pub fn new(path: &str) -> App {
        App {
            filename: path.to_string(),
            alignment: Alignment::new(read_fasta_file(path)
                           .expect(format!("File {} not found", path).as_str())),
            top_line: 0,
            leftmost_col: 0,
        }
    }

    pub fn scroll_one_line_up(&mut self) {
        if self.top_line > 0 { self.top_line -= 1; }
    }

    pub fn scroll_one_col_left(&mut self) {
        if self.leftmost_col > 0 { self.leftmost_col -= 1; }
    }
}
