// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Thomas Junier

// 0 <= v <= 1
pub fn values_barchart(values: &Vec<f64>) -> String {
    let mut blocks = String::with_capacity(values.len());
    for v in values {
        blocks.push(if *v <= 0.11111111111111112 {
            ' '
        } else if *v <= 0.22222222222222224 {
            '▁'
        } else if *v <= 0.3333333333333333 {
            '▂'
        } else if *v <= 0.4444444444444445 {
            '▃'
        } else if *v <= 0.5555555555555556 {
            '▄'
        } else if *v <= 0.6666666666666666 {
            '▅'
        } else if *v <= 0.7777777777777777 {
            '▆'
        } else if *v <= 0.888888888888889 {
            '▇'
        } else {
            // -> 1.0
            '█'
        });
    }
    blocks
}

pub fn value_to_hbar(v: f64) -> String {
    if v <= 0.0588 {
        "  ".to_string()
    } else if v <= 0.1176 {
        "▏ ".to_string() // 1/16 "217"
    } else if v <= 0.1765 {
        "▎ ".to_string() // 1/8
    } else if v <= 0.2353 {
        "▍ ".to_string()
    } else if v <= 0.2941 {
        "▌ ".to_string() // 1/4
    } else if v <= 0.3529 {
        "▋ ".to_string()
    } else if v <= 0.4118 {
        "▊ ".to_string() // 3/8
    } else if v <= 0.4706 {
        "▉ ".to_string()
    } else if v <= 0.5294 {
        "█ ".to_string() // 1/2
    } else if v <= 0.5882 {
        "█▏".to_string() // 9/16 "217"
    } else if v <= 0.6471 {
        "█▎".to_string() // 216
    } else if v <= 0.7059 {
        "█▍".to_string() // 215
    } else if v <= 0.7647 {
        "█▌".to_string() // 214 1/2
    } else if v <= 0.8235 {
        "█▋".to_string() // 213 5/8
    } else if v <= 0.8824 {
        "█▊".to_string() // 212 6/8
    } else if v <= 0.9412 {
        "█▉".to_string() // 211 7/8
    } else {
        // -> 1.0
        "██".to_string() // 210 full
    }
}

#[cfg(test)]
mod test {

    use crate::ui::barchart::{value_to_hbar, values_barchart};

    #[test]
    fn test_values_barchart() {
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let exp = " ▁▂▃▄▅▆▇█";
        assert_eq!(values_barchart(&values), exp);
    }

    #[test]
    fn test_h_barchart_00() {
        assert_eq!(value_to_hbar(0.0), "  ");
    }
}
