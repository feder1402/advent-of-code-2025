/***
 * Finds the position and value of the largest digit in a string slice
 * returns Option<(position, value)>
 * Examples:
 *  find_largest_digit("90") -> (0, 9)
 *  find_largest_digit("12") -> (1, 2)
 *  find_largest_digit("987654321111111") -> (0, 9)
 */
pub fn find_largest_digit(s: &str) -> Option<(usize, usize)> {
    let mut largest_digit: Option<(usize, usize)> = None;
    for (i, c) in s.chars().enumerate() {
        // println!("Checking ({}, {})", i, c);
        let digit = c.to_digit(10).unwrap() as usize;

        if largest_digit.is_none() || largest_digit.is_some_and(|(_, value)| digit > value) {
            //println!("Found largest digit {} at {}", digit, i);
            largest_digit = Some((i, digit));
        }
    }
    largest_digit
}

#[cfg(test)]
mod find_largest_digit_tests {
    use super::*;

    #[test]
    fn it_returns_none_for_empty_slice() {
        assert_eq!(find_largest_digit(&""), None);
    }

    #[test]
    fn it_returns_0_9_for_90() {
        assert_eq!(find_largest_digit(&"90"), Some((0, 9)));
    }

    #[test]
    fn it_returns_1_9_for_09() {
        assert_eq!(find_largest_digit(&"09"), Some((1, 9)));
    }

    #[test]
    fn it_returns_1_9_for_1988() {
        assert_eq!(find_largest_digit(&"1988"), Some((1, 9)));
    }

    #[test]
    fn it_returns_1_9_for_198899089() {
        assert_eq!(find_largest_digit(&"198899089"), Some((1, 9)));
    }

    #[test]
    fn it_returns_2_3_for_123() {
        assert_eq!(find_largest_digit(&"123"), Some((2, 3)));
    }

    #[test]
    fn it_returns_0_9_for_987654321111111() {
        assert_eq!(find_largest_digit(&"987654321111111"), Some((0, 9)));
    }

    #[test]
    fn it_returns_0_8_for_81111111111111() {
        assert_eq!(find_largest_digit(&"81111111111111"), Some((0, 8)));
    }
}
