pub fn entropies_to_blocks(entropies: &Vec<f64>) -> String {
    let mut blocks = String::with_capacity(entropies.len());
    let max = entropies.iter().fold(-1.0/0.0,
        |a:f64,b: &f64| f64::max(a, *b));
    for e in entropies {
        let rel_e: f64 = *e / max;
        println!("e: {}, r_e = {}/{} = {:?}", e, e, max, rel_e);
        blocks.push(
            if rel_e <= 0.11111111111111112  {
                ' '
            } else if rel_e <= 0.22222222222222224 {
                '▁'
            } else if rel_e <= 0.3333333333333333 {
                '▂'
            } else if rel_e <= 0.4444444444444445 {
                '▃'
            } else if rel_e <= 0.5555555555555556 {
                '▄'
            } else if rel_e <= 0.6666666666666666 {
                '▅'
            } else if rel_e <= 0.7777777777777777 {
                '▆'
            } else if rel_e <= 0.888888888888889 {
                '▇'
            } else { // -> 1.0
                '█'
            }
        );
    }
    blocks
}

mod test {
use crate::ui::conservation::entropies_to_blocks;

    #[test]
    fn test_entropies_to_blocks() {
        let entropies = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let exp = " ▁▂▃▄▅▆▇█";
        assert_eq!(entropies_to_blocks(&entropies), exp);
    }

}
