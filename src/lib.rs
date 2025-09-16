use std::array::from_fn;

// basic unsafe function.
unsafe fn concatenate<const N: usize, const M: usize, const R: usize, T>(first: [T;N], second: [T;M]) -> [T;R] {
    from_fn(|i| {
        if i<N {
            first[i]
        } else {
            second[i-N]
        }
    })
}

// value trait that automatatically gets implemented for any usize that satisfies the pred(s) (ie. that has summation(SELF) == true)
trait Sum<const N: usize, const M: usize> for usize {
    const pred summation(SELF) -> bool {
        N + M == SELF
    }
}

// safe wrapper of concatenate with appropriate where bounds making use of the value trait
fn safe_concatenate<const N: usize, const M: usize, const R: usize, T>(first: [T;N], second: [T;M]) -> [T;R]
where
    R: Sum<N,M>
{
    unsafe {
        concatenate(first,second)
    }
}

// here we use the trait Sum as a pattern — this would suggest it needing to be something more runtime than compile-time
// (which is fine since it's normally easier to map compile time things to runtime things).
fn is_sum_of(sum:usize, first:usize, second:usize) -> bool {
    match sum {
        // treat value trait as a (potentially complex) pattern
        Sum<first,second> => true,
        _ => false
    }
}

// runtime version
fn only_3_to_10(x:usize is 3..10) {
    assert!(x>=3);
    assert!(x<10);
}

// compile time version
fn only_3_to_10_alt<const X: usize>()
where
    // treat pattern as a value trait.
    X: 3..10
{
    only_3_to_10(X);
}

fn main() {
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

// associated playground:
// https://play.rust-lang.org/?version=stable&mode=debug&edition=2024&gist=55e5c77dda7fb3fe18a051550fe2207a