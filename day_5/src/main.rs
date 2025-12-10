use std::collections::{BTreeSet};
use std::ops::RangeInclusive;
use std::str::Lines;
use thousands::Separable;

#[derive(Debug, Clone)]
struct OrderedRange(RangeInclusive<i64>);

impl Ord for OrderedRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.start().cmp(other.0.start()) {
            std::cmp::Ordering::Equal => self.0.end().cmp(other.0.end()),
            other => other,
        }
    }
}

impl PartialOrd for OrderedRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for OrderedRange {}

impl PartialEq for OrderedRange {
    fn eq(&self, other: &Self) -> bool {
        self.0.start() == other.0.start() && self.0.end() == other.0.end()
    }
}

fn main() {
    let input = include_str!("../puzzle_input.txt");
    let lines = input.lines(); 

    let ordered_ranges = insert_into_ordered_list(lines);

    let mut nonov_ranges: Vec<OrderedRange> = Vec::new();
    for range in ordered_ranges {
        println!("Nonov size is {}. Merging {:?}", nonov_ranges.len(), range);

        if nonov_ranges.is_empty() { 
            println!("\tNonov empty. Inserting {:?}", range);
            nonov_ranges.push(range)
        } else {
            let last = nonov_ranges.pop().unwrap();
            println!("\tComparing with last: {:?}", last); 
            if find_intersection(&range, &last).is_none() {
                println!("\tNo intersection, inserting both: {:?}, and {:?}", last, range);
                nonov_ranges.push(last);
                nonov_ranges.push(range);
            } else {
                let new_range = find_union(&range, &last).unwrap();
                println!("\tRanges intersect. Inserting union: {:?}", new_range);
                nonov_ranges.push(new_range);
            }
        }
    }

    let mut fresh_ingredients: i64 = 0;
    for r in nonov_ranges {
        let range_ingredients= r.0.end() - r.0.start() + 1; 
        fresh_ingredients += range_ingredients;
         println!("Listing {} to {} - {} {}", r.0.start().separate_with_commas(), r.0.end().separate_with_commas(), range_ingredients.separate_with_commas(), fresh_ingredients.separate_with_commas());
    }

    println!("Fresh ingredients: {fresh_ingredients}");
}

fn insert_into_ordered_list(lines: Lines) -> BTreeSet<OrderedRange> {
        // Read ranges into a sorted list
        let mut ranges: BTreeSet<OrderedRange> = BTreeSet::new();

        for line in lines {
            if line.is_empty() { break };
            let values: Vec<&str> = line.split('-').collect();
            if values.len() != 2 {
                panic!("Invalid input line: {}", line);
            }
    
            let start = values[0].parse::<i64>().unwrap();
            let end = values[1].parse::<i64>().unwrap();

            if start >= end {
                println!("Found inverted range: {} - {}", start, end)
            }
    
            println!("Inserting range {start} to {end}");
            ranges.insert(OrderedRange(start..=end));
    
        }

        ranges
    
}

fn find_intersection(
    a: &OrderedRange,
    b: &OrderedRange,
) -> Option<OrderedRange> {
    if b.0.contains(a.0.start()) || b.0.contains(a.0.end()) {
        let new_start = if a.0.start() < b.0.start() {
            b.0.start()
        } else {
            a.0.start()
        };
        let new_end = if a.0.end() > b.0.end() { a.0.end() } else { b.0.end() };
        return Some(OrderedRange(RangeInclusive::new(*new_start, *new_end)));
    }

    None
}
fn find_union(a: &OrderedRange, b: &OrderedRange) -> Option<OrderedRange> {
    if b.0.contains(a.0.start()) || b.0.contains(a.0.end()) {
        let new_start = if a.0.start() < b.0.start() {
            a.0.start()
        } else {
            b.0.start()
        };
        let new_end = if a.0.end() > b.0.end() { a.0.end() } else { b.0.end() };
        return Some(OrderedRange(RangeInclusive::new(*new_start, *new_end)));
    }

    None
}

