pub fn subtract(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai - bi).collect()
}

pub fn add(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(ai, bi)| ai + bi).collect()
}

pub fn multiply(scalar: f64, vec: &[f64]) -> Vec<f64> {
    vec.iter().map(|&vi| scalar * vi).collect()
}

pub fn magnitude(vec: &[f64]) -> f64 {
    vec.iter().map(|&vi| vi * vi).sum::<f64>().sqrt()
}

pub fn normalize(vec: &[f64]) -> Vec<f64> {
    let mag = magnitude(vec);
    vec.iter().map(|&vi| vi / mag).collect()
}

pub fn orthogonalize(vec: &[f64], basis: &[Vec<f64>]) -> Vec<f64> {
    let mut result = vec.to_vec();
    for b in basis {
        let dot_product = vec.iter().zip(b.iter()).map(|(vi, bi)| vi * bi).sum::<f64>();
        let proj = multiply(dot_product, b);
        result = subtract(&result, &proj);
    }
    result
}

pub fn gram_schmidt(basis: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let mut ortho_basis = Vec::new();
    for b in basis {
        let ortho_vec = orthogonalize(b, &ortho_basis);
        ortho_basis.push(normalize(&ortho_vec));
    }
    ortho_basis
}

// UnitTests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtract() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![3.0, 2.0, 1.0];
        let result = subtract(&a, &b);
        assert_eq!(result, vec![-2.0, 0.0, 2.0]);
    }

    #[test]
    fn test_add() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![3.0, 2.0, 1.0];
        let result = add(&a, &b);
        assert_eq!(result, vec![4.0, 4.0, 4.0]);
    }

    #[test]
    fn test_multiply() {
        let scalar = 2.0;
        let vec = vec![1.0, 2.0, 3.0];
        let result = multiply(scalar, &vec);
        assert_eq!(result, vec![2.0, 4.0, 6.0]);
    }

    #[test]
    fn test_magnitude() {
        let vec = vec![1.0, 2.0, 3.0];
        let result = magnitude(&vec);
        assert_eq!(result, 3.7416573867739413);
    }

    #[test]
    fn test_normalize() {
        let vec = vec![1.0, 2.0, 3.0];
        let result = normalize(&vec);
        assert_eq!(result, vec![0.2672612419124244, 0.5345224838248488, 0.8017837257372732]);
    }

    #[test]
    fn test_orthogonalize() {
        let vec = vec![1.0, 2.0, 3.0];
        let basis = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0]];
        let result = orthogonalize(&vec, &basis);
        // println!("{:?}", result);
        assert_eq!(result, vec![0.0, 0.0, 3.0]);
    }

    #[test]
    fn test_gram_schmidt() {
        let basis = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]];
        let result = gram_schmidt(&basis);
        assert_eq!(result, vec![
            vec![1.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0],
            vec![0.0, 0.0, 1.0]
        ]);
    }
}