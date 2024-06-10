use rasta::read_fasta_file;

use crate::alignment::Alignment;


pub struct App {
    pub filename: String,
    pub alignment: Alignment,
    pub top_line: u16,
    pub leftmost_col: u16,
    seq_para_height: u16,
    seq_para_width: u16,
}

impl App {
    pub fn new(path: &str) -> Result<App, std::io::Error> {
        let fasta_file = read_fasta_file(path)?;
        Ok(App {
            filename: path.to_string(),
            alignment: Alignment::new(fasta_file),
            top_line: 0,
            leftmost_col: 0,
            seq_para_height: 0,
            seq_para_width: 0,
        })
    }

    // Computed properties (TODO: could be set in a struct member, as they do not change)

    pub fn num_seq(&self) -> u16 { self.alignment.num_seq().try_into().unwrap() }

    pub fn aln_len(&self) -> u16 { self.alignment.aln_len().try_into().unwrap() }

    // Setting size (must be done after layout is solved) - this is layout-agnostic, i.e. the
    // height is th aactual number of lines displayable in the sequence widget, after taking into
    // account its size, the presence of borders, etc.

    pub fn set_seq_para_height(&mut self, height: u16) { self.seq_para_height = height; }

    pub fn set_seq_para_width(&mut self, width: u16) { self.seq_para_width = width; }

    // Scrolling

    pub fn scroll_one_line_up(&mut self) {
        if self.top_line > 0 { self.top_line -= 1; }
    }

    pub fn scroll_one_col_left(&mut self) {
        if self.leftmost_col > 0 { self.leftmost_col -= 1; }
    }

    fn max_top_line(&self) -> u16 {
        if self.num_seq() >= self.seq_para_height {
            self.num_seq() - self.seq_para_height
        } else {
            0
        }
    }

    fn max_leftmost_col(&self) -> u16 {
        if self.aln_len() >= self.seq_para_width {
            self.aln_len() - self.seq_para_width
        } else {
            0
        }
    }

    pub fn scroll_one_line_down(&mut self) {
      if self.top_line < self.max_top_line() { self.top_line += 1; }
    }

    pub fn scroll_one_col_right(&mut self) {
        if self.leftmost_col < self.max_leftmost_col() { self.leftmost_col += 1; }
    }

    pub fn jump_to_top(&mut self) { self.top_line = 0 }

    pub fn jump_to_begin(&mut self) { self.leftmost_col = 0 }

    pub fn jump_to_bottom(&mut self) { self.top_line = self.max_top_line() }

    pub fn jump_to_end(&mut self) { self.leftmost_col = self.max_leftmost_col() }
}
