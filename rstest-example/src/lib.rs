/*pub fn sub(left: i32, right: i32) -> i32 {
    left - right
}*/

#[cfg(test)]
mod tests {
    //use super::*;
    //use rstest::rstest;
    use mockall::predicate::*;
    use mockall::*;
    #[automock]
    trait MyTrait {
        fn foo(&self, x: u32) -> u32;
    }
    fn func_with_trait( // This function takes a trait object as an argument
        x: &dyn MyTrait, v: u32 // 
    ) -> u32 {
        x.foo(v)
    }
    #[test]
    fn main() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo().returning(|x| x + 1);
        assert_eq!(11, func_with_trait(&mock, 9));
    }
    /*
    #[rstest]
    #[case(10, 0, 10)]
    #[case(100, 5, 105)]
    fn test_sub(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(sub(a, b), expected);
    } */
}
