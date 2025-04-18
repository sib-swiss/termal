// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier
use std::fmt;

use log::debug;

use rasta::read_fasta_file;

use crate::{
    alignment::Alignment,
    app::SeqOrdering::{SOURCE_FILE, METRIC_INCR, METRIC_DECR},
    app::Metric::{PCT_ID_WRT_CONSENSUS, SEQ_LEN},
};

#[derive(Clone, Copy)]
pub enum SeqOrdering {
    SOURCE_FILE,
    METRIC_INCR,
    METRIC_DECR,
}

impl fmt::Display for SeqOrdering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sord = match self {
            SOURCE_FILE => '-', 
            METRIC_INCR => '↑',
            METRIC_DECR => '↓',
        };
        write!(f, "{}", sord)
    }
}

#[derive(Clone, Copy)]
pub enum Metric {
    PCT_ID_WRT_CONSENSUS,
    SEQ_LEN,
}

impl fmt::Display for Metric {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let metric = match self {
            PCT_ID_WRT_CONSENSUS => "%id (cons)", 
            SEQ_LEN => "seq len",
        };
        write!(f, "{}", metric)
    }
}

pub struct App {
    pub filename: String,
    pub alignment: Alignment,
    ordering_criterion: SeqOrdering,
    metric: Metric,
    // Specifies in which order the aligned sequences should be displayed. The elements of this Vec
    // are _indices_ into the Vec's of headers and sequences that together make up the alignment.
    // By default, they are just ordered from 1 to aln-width - 1, but the user can choose to order
    // according to the current metric, in which case the ordering becomes that of the metric's
    // value for each sequence.
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
            metric: PCT_ID_WRT_CONSENSUS,
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

    fn recompute_ordering(&mut self) {
        match self.ordering_criterion {
            METRIC_INCR => {
                self.ordering = order(&self.order_values());
            }
            METRIC_DECR => {
                let mut ord = order(&self.order_values());
                ord.reverse();
                self.ordering = ord;
            }
            SOURCE_FILE => {
                self.ordering = (0..self.alignment.num_seq()).collect();
            }
        }
    }

    pub fn cycle_ordering_criterion(&mut self) {
        self.ordering_criterion = match self.ordering_criterion {
            SOURCE_FILE => METRIC_INCR,
            METRIC_INCR => METRIC_DECR,
            METRIC_DECR => SOURCE_FILE,
        };
        self.recompute_ordering();
    }

    pub fn cycle_metric(&mut self) {
        self.metric = match self.metric {
            PCT_ID_WRT_CONSENSUS =>  SEQ_LEN,
            SEQ_LEN => PCT_ID_WRT_CONSENSUS,
        };
        self.recompute_ordering();
    }

    pub fn output_info(&self) {
        println!("name: {}", self.filename);
        println!("nb_sequences: {}", self.num_seq());
        println!("nb_columns: {}", self.aln_len());
        println!();
    }

    pub fn get_seq_ordering(&self) -> SeqOrdering {
        self.ordering_criterion
    }

    pub fn get_metric(&self) -> Metric {
        self.metric
    }

    pub fn order_values(&self) -> &Vec<f64> {
         match self.metric {
            PCT_ID_WRT_CONSENSUS => &self.alignment.id_wrt_consensus,
            SEQ_LEN => &self.alignment.relative_seq_len, 
        }
    }
}

// Computes an ordering WRT an array, that is, an array of indices of elements of the source array,
// after sorting. Eg [3, -2, 7] -> [1, 0, 2], because the smalllest element has index 1, the next
// has index 0, and the largest has index 2 (in the original array).
fn order(nums: &Vec<f64>) -> Vec<usize> {
    // let result: Vec<usize> = Vec::with_capacity(nums.len());
    let init_order: Vec<usize> = (0..nums.len()).collect();
    let zip_iter = init_order.iter().zip(nums);
    let mut unsorted_pairs: Vec<(&usize, &f64)> = zip_iter.collect();
    unsorted_pairs.sort_by(|(_, t1), (_, t2)| t1.partial_cmp(t2).expect("Unorder!"));
    unsorted_pairs.into_iter().map(|(u, _)| *u).collect::<Vec<usize>>()
}

#[cfg(test)]
mod tests {

    use crate::app::order;

    #[test]
    fn test_order_00() {
        assert_eq!(
            vec![2,1,0],
            order(&vec![20.0, 15.0, 10.0])
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
