pub fn sub(left: i32, right: i32) -> i32 {
    left - right
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(10, 0, 10)]
    #[case(100, 5, 105)]
    fn test_sub(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(sub(a, b), expected);
    }
}
