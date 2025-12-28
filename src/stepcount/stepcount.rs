// Source: https://www.youtube.com/watch?v=JXUOMsFBDXQ
// Returns a 1 if the sequence found is valid
// Returns a 0 if the sequence found is invalid
pub fn stepcount(n: i32, steps: Vec<i32>) -> i32 {
    if n == 0 {
        return 1;
    }

    if n < 0 {
        return 0;
    }

    steps.iter().map(|&s| stepcount(n - s, steps.clone())).sum()
}

#[test]
fn test_stepcount() {
    // Arrange
    let n = 10;
    let steps = vec![1, 3, 5];

    // Act
    let result = stepcount(n, steps);

    // Assert
    assert_eq!(result, 47);
}