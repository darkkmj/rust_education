// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {
    let a: Option<i32> = None;
    dbg!(a);

    let a_is_some = a.is_some();
    dbg!(a_is_some);

    let a_is_none = a.is_none();
    dbg!(a_is_none);

    let a_mapped = a.map(|num| num + 1);
    dbg!(a_mapped);

    let a_filtered = a.filter(|num| num == &1);
    dbg!(a_filtered);

    let a_or_else = a.or_else(|| Some(5));
    dbg!(a_or_else);

    let a_unwrapped = a.unwrap_or_else(|| 0);
    dbg!(a_unwrapped);
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn clamp_test_1() {
        let result = clamp(10, 2, 20);
        assert_eq!(result, 10, "Invalid result(clamp)");
    }

    #[test]
    fn clamp_test_2() {
        let result = clamp(2, 2, 20);
        assert_eq!(result, 2, "Invalid result(clamp)");
    }

    #[test]
    fn div_normal() {
        let result = div(10, 2);
        assert_eq!(result, Some(5), "Invalid normal result(div)");
    }

    #[test]
    fn div_abnormal() {
        let result = div(10, 0);
        assert_eq!(result, None, "Invalid abnormal result(div)");
    }

    #[test]
    fn concat_test_1() {
        let msg = concat("1", "2");
        assert_eq!(msg, "1 2", "Invalid result(concat)");
    }

    #[test]
    fn concat_test_2() {
        let msg = concat("3", "");
        assert_eq!(msg, "3 ", "Invalid result(concat)");
    }
}