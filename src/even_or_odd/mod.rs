pub fn even_or_odd(number: i32) -> &'static str {
    match number % 2 {
        0 => "Even",
        1 | -1 => "Odd",
        _ => "ERROR"
    }
}

fn do_test(number: i32, expected: &str) {
    let actual = even_or_odd(number);
    assert_eq!(actual, expected, "\nYour result (left) does not match the expected output (right) for the input {number:?}");
}

#[cfg(test)]
mod fixed_tests {
    use super::*;

    #[test]
    fn test_zero() {
        do_test(0, "Even");
    }

    #[test]
    fn test_positive_even() {
        do_test( 2, "Even");
        do_test(20, "Even");
    }

    #[test]
    fn test_positive_odd() {
        do_test( 1, "Odd");
        do_test(21, "Odd");
    }

    #[test]
    fn test_negative_even() {
        do_test( -2, "Even");
        do_test(-20, "Even");
    }

    #[test]
    fn test_negative_odd() {
        do_test( -1, "Odd");
        do_test(-21, "Odd");
    }
}

// Tests from codewars.com/kata/53da3dbb4a5168369a0000fe/solutions/rust

#[cfg(test)]
mod random_tests {
    use super::*;
    use rand::Rng;
    use rand::seq::SliceRandom;

    fn generate_cases() -> Vec<(i32, &'static str)> {
        let mut cases: Vec<(i32, &str)> =
            (0..25).flat_map(|_| {
                let b: i32 = rand::thread_rng().gen_range(1..=10_000);
                vec![
                    (b *  2,     "Even"),
                    (b *  2 + 1, "Odd"),
                    (b * -2,     "Even"),
                    (b * -2 - 1, "Odd")]
            }).collect();
        cases.shuffle(&mut rand::thread_rng());
        cases
    }

    #[test]
    fn test_random_inputs() {
        let test_cases = generate_cases();
        for (number, expected) in test_cases {
            do_test(number, expected);
        }
    }
}