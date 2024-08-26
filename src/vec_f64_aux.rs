// Subtracts the minimum and then divides by the maximum -> [0, 1]
pub fn normalize(values: &[f64]) -> Vec<f64> {
    let max = values.iter().fold(-1.0/0.0,
        |a:f64,b: &f64| f64::max(a, *b));
    let min = values.iter().fold(1.0/0.0,
        |a:f64,b: &f64| f64::min(a, *b));
    values.iter().map(|v| (v-min) / (max-min)).collect()
}

// Complements to one. Only makes sense for normalized values.
pub fn ones_complement(values: &[f64]) -> Vec<f64> {
    values.iter().map(|v| 1.0 - v).collect()
}


// Element-wise product of two Vec<f64>
pub fn product(v1: &[f64], v2: &[f64]) -> Vec<f64> {
    v1.iter().zip(v2).map(|(v, w)| v * w).collect()
}

mod test {
    use approx::assert_relative_eq;
    use crate::vec_f64_aux::{
        normalize,
        ones_complement,
        product,
    };

    #[test]
    fn test_normalize_1() {
        let values = vec![0.0, 2.0, 4.0];
        let exp = vec![0.0, 0.5, 1.0];
        assert_eq!(normalize(&values), exp);
    }

    #[test]
    fn test_normalize_2() {
        let values = vec![-4.0, -2.0, 0.0, 2.0, 4.0];
        let exp    = vec![0.0, 0.25, 0.5, 0.75, 1.0];
        assert_eq!(normalize(&values), exp);
    }

    #[test]
    fn test_ones_complement_1() {
        let values = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let comps  = ones_complement(&values);
        let exp    = vec![0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2, 0.1];
        for (c, e) in comps.iter().zip(exp) {
            assert_relative_eq!(*c, e, epsilon=0.001);
        }
    }
    
    #[test]
    fn test_product() {
        let v1 = vec![1.0, 2.0, -3.0];
        let v2 = vec![1.0, 0.5, 3.0];
        let exp = vec![1.0, 1.0, -9.0];
        assert_eq!(exp, product(&v1, &v2));
    }

}
