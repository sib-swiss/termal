// TODO: this module should be renamed "barchart", as it is used for purposes other than
// conservation (such as sequence metrics).

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
    if v <= 0.0556 {
        "  "
    } else if v <= 0.1111 {
        "▏ "
    } else if v <= 0.1667 {
        "▎ " 
    } else if v <= 0.2222 {
        "▍ " 
    } else if v <= 0.2778 {
        "▌ " 
    } else if v <= 0.3333 {
        "▋ " 
    } else if v <= 0.3889 {
        "▊ " 
    } else if v <= 0.4444 {
        "▉ " 
    } else if v <= 0.5 {
        "▉▏ "
    } else if v <= 0.5556 {
        "▉▎"
    } else if v <= 0.6111 {
        "▉▍"
    } else if v <= 0.6667 {
        "▉▌"
    } else if v <= 0.7222 {
        "▉▋"
    } else if v <= 0.7778
        "▉▊"
    } else if v <= 0.8333 {
        "▉"
    } else {
        // -> 1.0
        "█"
    }
}

#[cfg(test)]
mod test {

    use crate::ui::conservation::values_barchart;

    #[test]
    fn test_values_barchart() {
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let exp = " ▁▂▃▄▅▆▇█";
        assert_eq!(values_barchart(&values), exp);
    }
}
