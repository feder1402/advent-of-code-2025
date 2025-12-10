use day_2::IdRange;

fn main() {
    let ranges_file = include_str!("../ranges.txt");

    let mut invalid_ids: Vec<u64> = Vec::new();

    for range_str in ranges_file.split(',') {
        let range = IdRange::from(range_str.to_string());
        invalid_ids.extend(range.find_new_invalids());
    }

    let sum = invalid_ids.iter().sum::<u64>();

    println!("Sum of invalid IDs: {}", sum);
}
