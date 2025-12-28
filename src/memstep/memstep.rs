use std::collections::HashMap;
use num_bigint::BigInt;
use num_traits::{One, Zero};

pub fn memstep(n: i32, steps: Vec<i32>) -> BigInt {
    memstep_cached(n, &steps, &mut HashMap::new())
}

fn memstep_cached(n: i32, steps: &Vec<i32>, cache: &mut HashMap<i32, BigInt>) -> BigInt {
    if n == 0 {
        return BigInt::one();
    }

    if n < 0 {
        return BigInt::zero();
    }

    if let Some(result) = cache.get(&n) {
        return result.clone();
    }

    let result: BigInt = steps.iter()
        .map(|&s| memstep_cached(n - s, steps, cache))
        .sum();

    cache.insert(n, result.clone());
    result
}

#[test]
fn test_memstep() {
    // Arrange
    let n = 10;
    let steps = vec![1, 3, 5];

    // Act
    let result = memstep(n, steps);

    // Assert
    assert_eq!(result, BigInt::from(47));
}