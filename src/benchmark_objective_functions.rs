use std::f64::consts::PI;


pub fn simple_objective(input: &Vec<f64>) -> Vec<f64> {
    // sum of squares
    let result1 = input.iter().map(|x| x * x).sum::<f64>();
    let result2 = input.iter().map(|x| x * x - x ).sum::<f64>();

    vec![result1, result2]
}

pub fn xyz_objective(input: &Vec<f64>) -> Vec<f64> {
    let x = input[0];
    let y = input[1];
    let z = input[2];
    
    let x_squared = x * x;
    let y_squared = y * y; 
    let z_squared: f64 = z * z;
    let v1 = &x_squared + &y_squared;
    let v2: f64 = &x_squared - &y_squared - &z_squared;
    let result = vec![v1, v2];
    result 
}

pub fn dtlz1(input: &Vec<f64>) -> Vec<f64> {
    let n = input.len();
    let k = n - 5;
    let g = 100.0 * (k as f64 + input.iter().skip(n - k).map(|&x| (x - 0.5) * (x - 0.5) - (20.0 * PI * (x - 0.5)).cos()).sum::<f64>());
    let mut f = vec![0.5 * (1.0 + g); n - 1];
    for i in 0..(n - 1) {
        f[i] = 0.5 * (1.0 + g) * input.iter().take(n - i - 1).product::<f64>();
    }
    f
}

pub fn dtlz2(input: &Vec<f64>) -> Vec<f64> {
    let n = input.len();
    let k = n - 5;
    let g = input.iter().skip(n - k).map(|&x| (x - 0.5) * (x - 0.5)).sum::<f64>();
    let mut f = vec![1.0 + g; n - 1];
    for i in 0..(n - 1) {
        f[i] = (1.0 + g) * input.iter().take(n - i - 1).map(|&x| x.cos()).product::<f64>() * input[n - i - 2].sin();
    }
    f
}

pub fn dtlz3(input: &Vec<f64>) -> Vec<f64> {
    let n = input.len();
    let k = n - 5;
    let g = 100.0 * (k as f64 + input.iter().skip(n - k).map(|&x| (x - 0.5) * (x - 0.5) - (20.0 * PI * (x - 0.5)).cos()).sum::<f64>());
    let mut f = vec![1.0 + g; n - 1];
    for i in 0..(n - 1) {
        f[i] = (1.0 + g) * input.iter().take(n - i - 1).map(|&x| x.cos()).product::<f64>() * input[n - i - 2].sin();
    }
    f
}

pub fn dtlz4(input: &Vec<f64>, alpha: f64) -> Vec<f64> {
    let n = input.len();
    let k = n - 5;
    let g = input.iter().skip(n - k).map(|&x| (x - 0.5) * (x - 0.5)).sum::<f64>();
    let mut f = vec![1.0 + g; n - 1];
    for i in 0..(n - 1) {
        f[i] = (1.0 + g) * input.iter().take(n - i - 1).map(|&x| x.powf(alpha).cos()).product::<f64>() * input[n - i - 2].powf(alpha).sin();
    }
    f
}

pub fn dtlz5(input: &Vec<f64>) -> Vec<f64> {
    let n = input.len();
    let k = n - 5;
    let g = input.iter().skip(n - k).map(|&x| (x - 0.5) * (x - 0.5)).sum::<f64>();
    let mut theta = vec![0.0; n - 1];
    theta[0] = input[0] * PI / 2.0;
    for i in 1..(n - 1) {
        theta[i] = (1.0 + 2.0 * g * input[i]).atan() / (4.0 * (1.0 + g));
    }
    let mut f = vec![1.0 + g; n - 1];
    for i in 0..(n - 1) {
        f[i] = (1.0 + g) * theta.iter().take(n - i - 1).map(|&t| t.cos()).product::<f64>() * theta[n - i - 2].sin();
    }
    f
}

pub fn dtlz6(input: &Vec<f64>) -> Vec<f64> {
    let n = input.len();
    let k = n - 5;
    let g = input.iter().skip(n - k).map(|&x| x.powf(0.1)).sum::<f64>();
    let mut theta = vec![0.0; n - 1];
    theta[0] = input[0] * PI / 2.0;
    for i in 1..(n - 1) {
        theta[i] = (1.0 + 2.0 * g * input[i]).atan() / (4.0 * (1.0 + g));
    }
    let mut f = vec![1.0 + g; n - 1];
    for i in 0..(n - 1) {
        f[i] = (1.0 + g) * theta.iter().take(n - i - 1).map(|&t| t.cos()).product::<f64>() * theta[n - i - 2].sin();
    }
    f
}

pub fn dtlz7(input: &Vec<f64>) -> Vec<f64> {
    let n = input.len();
    let k = n - 5;
    let g = 1.0 + 9.0 * input.iter().skip(n - k).sum::<f64>() / k as f64;
    let mut f = vec![0.0; n - 1];
    for i in 0..(n - 1) {
        f[i] = input[i];
    }
    let h = (n - 1) as f64 - f.iter().map(|&x| x / (1.0 + g) * (1.0 + (2.0 * PI * x).sin())).sum::<f64>();
    f.push((1.0 + g) * h);
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dtlz1() {
        let input = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let result = dtlz1(&input);
        assert_eq!(result.len(), input.len() - 1);
        // Add specific checks based on expected results
    }

    #[test]
    fn test_dtlz2() {
        let input = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let result = dtlz2(&input);
        assert_eq!(result.len(), input.len() - 1);
        // Add specific checks based on expected results
    }

    #[test]
    fn test_dtlz3() {
        let input = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let result = dtlz3(&input);
        assert_eq!(result.len(), input.len() - 1);
        // Add specific checks based on expected results
    }

    #[test]
    fn test_dtlz4() {
        let input = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let result = dtlz4(&input, 100.0);
        assert_eq!(result.len(), input.len() - 1);
        // Add specific checks based on expected results
    }

    #[test]
    fn test_dtlz5() {
        let input = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let result = dtlz5(&input);
        assert_eq!(result.len(), input.len() - 1);
        // Add specific checks based on expected results
    }

    #[test]
    fn test_dtlz6() {
        let input = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let result = dtlz6(&input);
        assert_eq!(result.len(), input.len() - 1);
        // Add specific checks based on expected results
    }

    #[test]
    fn test_dtlz7() {
        let input = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let result = dtlz7(&input);
        assert_eq!(result.len(), input.len());
        // Add specific checks based on expected results
    }
}


