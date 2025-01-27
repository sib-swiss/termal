use rasta::read_fasta_file;

use crate::{
    alignment::Alignment,
    app::SeqOrdering::{SOURCE_FILE, METRIC_INCR, METRIC_DECR},
};

enum SeqOrdering {
    SOURCE_FILE,
    METRIC_INCR,
    METRIC_DECR,
}

pub struct App {
    pub filename: String,
    pub alignment: Alignment,
    ordering_criterion: SeqOrdering,
    pub ordering: Vec<usize>,
}

impl App {
    pub fn new(path: &str) -> Result<App, std::io::Error> {
        let fasta_file = read_fasta_file(path)?;
        let alignment =  Alignment::new(fasta_file);
        let len = alignment.num_seq();
        Ok(App {
            filename: path.to_string(),
            alignment,
            ordering_criterion: SOURCE_FILE,
            ordering: (0..len).collect(),
        })
    }

    // Computed properties (TODO: could be set in a struct member, as they do not change)
    // FIXME where do we need num_seq as u16?

    pub fn num_seq(&self) -> u16 {
        self.alignment.num_seq().try_into().unwrap()
    }

    pub fn aln_len(&self) -> u16 {
        self.alignment.aln_len().try_into().unwrap()
    }

    pub fn cycle_ordering(&mut self) {
        match self.ordering_criterion {
            SOURCE_FILE => {
                // Next criterion is according to metric
                self.ordering_criterion = METRIC_INCR;
                self.ordering = order(&self.alignment.id_wrt_consensus);
            }
            METRIC_INCR => {
                // Next criterion is according to metric, descending
                self.ordering_criterion = METRIC_DECR;
                let mut ord = order(&self.alignment.id_wrt_consensus);
                ord.reverse();
                self.ordering = ord;
            }
            METRIC_DECR => {
                self.ordering_criterion = SOURCE_FILE;
                self.ordering = (0..self.alignment.num_seq()).collect();
                assert_eq!(self.num_seq(), self.ordering.len() as u16);
            }

        }
    }

    pub fn output_info(&self) {
        println!("name: {}", self.filename);
        println!("nb_sequences: {}", self.num_seq());
        println!("nb_columns: {}", self.aln_len());
        println!();
    }
}

fn order<T: PartialOrd+Copy>(nums: &Vec<T>) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::with_capacity(nums.len());
    let init_order: Vec<usize> = (0..nums.len()).collect();
    let mut zip_iter = init_order.iter().zip(nums);
    let mut unsorted_pairs: Vec<(&usize, &T)> = zip_iter.collect();
    unsorted_pairs.sort_by(|(u1, t1), (u2, t2)| t1.partial_cmp(t2).expect("Unorder!"));
    unsorted_pairs.into_iter().map(|(u, t)| *u).collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {

    use crate::app::order;

    #[test]
    fn test_order_00() {
        assert_eq!(
            vec![2,1,0],
            order(&vec![20, 15, 10])
            );
    }

    #[test]
    fn test_order_05() {
        assert_eq!(
            vec![3, 2, 0, 1, 4],
            order(&vec![12.23, 34.89, 7.0, -23.2, 100.0]),
            );
    }
}
