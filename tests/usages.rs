use value_traits_trials::safe_concatenate;

#[test]
fn test_safe_concatenate() {
    let first = [1, 2, 3];
    let second = [4, 5];
    /* 
    the trait solver would go:
    1. safe_concatenate needs 5 implements Sum<3,2>
    2. prove this by deferred predicate handling:
        - transform the 5: Sum<3,2> requirement into a Validate(Sum<3,2>::summation(5)) object
        - this is to be handled once summation can be defined as a function `usize -> bool` using the constants 3,2 internally.
    */
    let result: [i32; 5] = safe_concatenate(first, second);
    assert_eq!(result, [1, 2, 3, 4, 5]);
}