/***
 * Finds the power in jolts for a bnk of batteries from its label (see https://adventofcode.com/2025/day/3)
 * Input:
 *  - a string containing the label of the batteries
 *  - the exact number of batteries to activate
 * Returns: the battery power in jolts, calculated by concatenating the two largest digits of the label without altering their order
 *
 * Examples:
 *
 * - find_battery_power("987654321111111") -> 98
 * - find_battery_power("811111111111119") -> 89
 * - find_battery_power("234234234234278") -> 78
 * - find_battery_power("818181911112111") -> 92
 *
 */
use crate::libs::find_largest_digit;

pub fn find_battery_power(label: String, num_batteries: usize) -> usize {
    // Label should have at least num_battery digits
    if label.len() < num_batteries {
        return 0;
    }

    let mut battery_power: usize = 0;
    let mut start = 0;
    for digit_number in 1..=num_batteries {
        let end = label.len() - num_batteries + digit_number;
        let (pos, max_digit) = find_largest_digit::find_largest_digit(&label[start..end]).unwrap();
        println!("Max_digit {}, at {}, start {}, end {}", max_digit, pos, start, end);

        battery_power = battery_power * 10 + max_digit;
        start += pos + 1;
    }
    // // Find the position and value of the first largest digit
    // // Exclude the last digit in the label as we need to find two digits
    // let (pos, first_digit) = find_largest_digit::find_largest_digit(&label[0..label.len()-1]).unwrap();
    //
    // // Find the second digit, starting from the position after the first largest digit
    // let (_, second_digit) = find_largest_digit::find_largest_digit(&label[pos+1..]).unwrap();

    // Return the power in jolts
    battery_power
}

#[cfg(test)]
mod find_battery_power_tests {
    use super::*;

    #[test]
    fn it_returns_0_for_empty_label() {
        assert_eq!(find_battery_power("".to_string(), 2), 0);
    }

    #[test]
    fn it_returns_98_for_987654321111111() {
        assert_eq!(find_battery_power("987654321111111".to_string(), 2), 98);
    }

    #[test]
    fn it_returns_89_for_811111111111119() {
        assert_eq!(find_battery_power("811111111111119".to_string(), 2), 89);
    }

    #[test]
    fn it_returns_98_for_8111111111111199() {
        assert_eq!(find_battery_power("8111111111111199".to_string(), 2), 99);
    }

    #[test]
    fn it_returns_99_for_99() {
        assert_eq!(find_battery_power("99".to_string(), 2), 99);
    }

    #[test]
    fn it_returns_78_for_234234234234278() {
        assert_eq!(find_battery_power("234234234234278".to_string(), 2), 78);
    }

    #[test]
    fn it_returns_92_for_818181911112111() {
        assert_eq!(find_battery_power("818181911112111".to_string(), 2), 92);
    }

    #[test]
    fn it_returns_987654321111_for_987654321111111() {
        assert_eq!(find_battery_power("987654321111111".to_string(), 12), 987_654_321_111);
    }

    #[test]
    fn it_returns_811111111119_for_811111111111119() {
        assert_eq!(find_battery_power("811111111111119".to_string(), 12), 811_111_111_119);
    }

    #[test]
    fn it_returns_434234234278_for_234234234234278() {
        assert_eq!(find_battery_power("234234234234278".to_string(), 12), 434_234_234_278);
    }

    #[test]
    fn it_returns_888911112111_for_818181911112111() {
        assert_eq!(find_battery_power("818181911112111".to_string(), 12), 888_911_112_111);
    }

}
