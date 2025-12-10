use std::ops::RangeInclusive;
use substring::Substring;

pub struct IdRange {
    start: u64,
    end: u64,
}

impl IdRange {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }

    pub fn find_invalids(&self) -> Vec<u64> {
        let mut invalid_ids: Vec<u64> = Vec::new();
        let range = RangeInclusive::new(self.start, self.end);

        for id in range {
            let id = id.to_string();

            // Has leading zeroes
            if id.chars().nth(0) == Some('0') {
                invalid_ids.push(id.parse::<u64>().unwrap());
            }

            // Is repeated sequence
            if id.len() % 2 == 0 {
                let s1 = &id[0..id.len() / 2];
                let s2 = &id[id.len() / 2..];
                if s1 == s2 {
                    invalid_ids.push(id.parse::<u64>().unwrap());
                }
            }
        }

        invalid_ids
    }

    pub fn find_new_invalids(&self) -> Vec<u64> {
        let mut invalid_ids: Vec<u64> = Vec::new();
        let range = RangeInclusive::new(self.start, self.end);

        for id in range {
            let id = id.to_string();

            // Is repeated sequence
            if IdRange::is_repeated_sequence(&id) {
                invalid_ids.push(id.parse::<u64>().unwrap());
            }
        }

        invalid_ids
    }

    pub fn is_repeated_sequence(id: &str) -> bool {
        let len = id.len();

        for size in 1..=(len / 2) {
            if len % size != 0 { continue }

            let mut parts: Vec<String> = Vec::new();
            for i in 0..(len / size)  {
                parts.push(id.substring(i * size,(i + 1) * size).parse().unwrap())
            };
            let pattern = &parts[0];

            if parts.iter().all( |c| c == pattern) { return true }
        }

        false
    }
}

impl From<String> for IdRange {
    fn from(range_str: String) -> Self {
        let (start, end) = range_str.split_once("-").unwrap();

        let start = start.parse::<u64>().unwrap();
        let end = end.parse::<u64>().unwrap();

        IdRange::new(start, end)
    }
}

#[cfg(test)]
mod tests {
    use crate::IdRange;

    #[test]
    fn it_builds_a_11_to_22_range() {
        let range = IdRange::from("11-22".to_string());
        assert_eq!(range.start, 11);
        assert_eq!(range.end, 22);
    }

    #[test]
    fn it_builds_a_big_range() {
        let range = IdRange::from("9494926669-9494965937".to_string());
        assert_eq!(range.start, 9_494_926_669);
        assert_eq!(range.end, 9_494_965_937);
    }

    #[test]
    fn it_find_2_invalid_ids_for_11_to_22_range() {
        let range = IdRange::from("11-22".to_string());
        let invalid_ids = range.find_invalids();
        assert_eq!(invalid_ids.len(), 2);
        assert!(invalid_ids.contains(&11));
        assert!(invalid_ids.contains(&22));
    }

    #[test]
    fn it_find_2_new_invalid_ids_for_11_to_22_range() {
        let range = IdRange::from("11-22".to_string());
        let invalid_ids = range.find_new_invalids();
        assert_eq!(invalid_ids.len(), 2);
        assert!(invalid_ids.contains(&11));
        assert!(invalid_ids.contains(&22));
    }

    #[test]
    fn it_find_1_invalid_ids_for_95_to_115_range() {
        let range = IdRange::from("95-115".to_string());
        let invalid_ids = range.find_invalids();
        assert_eq!(invalid_ids.len(), 1);
        assert!(invalid_ids.contains(&99));
    }

    #[test]
    fn it_find_1_new_invalid_ids_for_95_to_115_range() {
        let range = IdRange::from("95-115".to_string());
        let invalid_ids = range.find_new_invalids();
        assert_eq!(invalid_ids.len(), 1);
        assert!(invalid_ids.contains(&99));
    }

    #[test]
    fn it_find_1_invalid_ids_for_118xx_to_118xx_range() {
        let range = IdRange::from("1188511880-1188511890".to_string());
        let invalid_ids = range.find_invalids();
        assert_eq!(invalid_ids.len(), 1);
        assert!(invalid_ids.contains(&1_188_511_885));
    }

    #[test]
    fn it_finds_sequence_in_1188511885() {
        let is_invalid =  IdRange::is_repeated_sequence(&"1188511885".to_string());
        assert!(is_invalid);
    }

    #[test]
    fn it_find_1_new_invalid_ids_for_118xx_to_118xx_range() {
        let range = IdRange::from("1188511880-1188511890".to_string());
        let invalid_ids = range.find_new_invalids();
        assert_eq!(invalid_ids.len(), 1);
        assert!(invalid_ids.contains(&1_188_511_885));
    }

    #[test]
    fn it_find_1_invalid_ids_for_222xx_to_222xx_range() {
        let range = IdRange::from("222220-222224".to_string());
        let invalid_ids = range.find_invalids();
        assert_eq!(invalid_ids.len(), 1);
        assert!(invalid_ids.contains(&222_222));
    }

    #[test]
    fn it_find_no_invalid_ids_for_169xx_to_169xx_range() {
        let range = IdRange::from("1698522-1698528".to_string());
        let invalid_ids = range.find_invalids();
        assert_eq!(invalid_ids.len(), 0);
    }

    #[test]
    fn it_find_1_invalid_ids_for_446xx_to_446xx_range() {
        let range = IdRange::from("446443-446449".to_string());
        let invalid_ids = range.find_invalids();
        assert_eq!(invalid_ids.len(), 1);
        assert!(invalid_ids.contains(&446_446));
    }

    #[test]
    fn it_find_1_invalid_ids_for_385xx_to_385xx_range() {
        let range = IdRange::from("38593856-38593862".to_string());
        let invalid_ids = range.find_invalids();
        assert_eq!(invalid_ids.len(), 1);
        assert!(invalid_ids.contains(&38_593_859));
    }

    #[test]
    fn it_calculates_the_sum_of_invalid_ids() {
        let ranges = "\
11-22,\
95-115,\
998-1012,\
1188511880-1188511890,\
222220-222224,\
1698522-1698528,\
446443-446449,\
38593856-38593862,\
565653-565659,\
824824821-824824827,\
2121212118-2121212124";

        let mut invalid_ids: Vec<u64> = Vec::new();

        for range_str in ranges.split(',') {
            let range = IdRange::from(range_str.to_string());
            invalid_ids.extend(range.find_invalids());
        }

        let sum = invalid_ids.iter().sum::<u64>();

        assert_eq!(sum, 1_227_775_554);
    }
}
