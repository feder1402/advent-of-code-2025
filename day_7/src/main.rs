use std::fs;

#[allow(unused)]
fn main() {
    let mut manifold: Vec<Vec<char>> = load_manifold("puzzle_input.txt");

    let mut splits: usize = 0;
    let mut beams: Vec<usize> = vec![];

    for manifold_layer in manifold.iter_mut() {
        // Insert the beams into the current layer
        for beam in beams {
            insert_beam(beam, &mut splits, manifold_layer);
        }

        // Find the positions of the beams in the current layer, including the initial 'S' beam
        beams = manifold_layer
            .iter()
            .enumerate()
            .filter(|item| ['S', '|'].contains(item.1))
            .map(|item| item.0)
            .collect::<Vec<usize>>();
    }

    println!("Diagram:");
    for r in &manifold {
        let row: String = r.iter().collect();
        println!("{}", row);
    }
    println!("Splits: {splits}");

    // Get number of timelines for part two of the problem
    let mut extended_manifold = manifold.clone();
    extended_manifold.push(vec!['.'; manifold[0].len()]);
    let beam_positions: Vec<usize> = extended_manifold[0]
        .iter()
        .enumerate()
        .filter(|item| ['S', '|'].contains(item.1))
        .map(|item| item.0)
        .collect();
    let mut memo: Vec<Vec<Option<usize>>> = (0..extended_manifold.len())
        .map(|i| vec![None; extended_manifold[i].len()])
        .collect();
    let mut sum: usize = 0;
    for &beam_position in &beam_positions {
        sum += count_timelines(beam_position, 0, &extended_manifold, &mut memo);
    }
    println!("Timelines: {sum}");
}

fn count_timelines(
    column: usize,
    row: usize,
    manifold: &Vec<Vec<char>>,
    memo: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if row == manifold.len() {
        return 1; // one completed timeline (leaf)
    }

    if let Some(v) = memo[row][column] {
        return v;
    }

    let result = if manifold[row][column] == '^' {
        let mut sum = 0;
        if column > 0 {
            sum += count_timelines(column - 1, row + 1, manifold, memo)
        };
        if column < (manifold[row].len() - 1) {
            sum += count_timelines(column + 1, row + 1, manifold, memo)
        };
        sum
    } else {
        count_timelines(column, row + 1, manifold, memo)
    };

    memo[row][column] = Some(result);

    result
}

fn load_manifold(file_path: &str) -> Vec<Vec<char>> {
    let input = fs::read_to_string(file_path).unwrap();

    let lines = input.lines();

    lines
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect()
}

// If the position has:
//  - empty space: insert a beam at the current position
//  - a splitter: split the beam to the adjacent (left and right) positions
fn insert_beam(position: usize, splits: &mut usize, row_chars: &mut Vec<char>) {
    match row_chars[position] {
        '.' => row_chars[position] = '|',
        '^' => {
            *splits += 1;
            if position > 0 {
                insert_beam(position - 1, splits, row_chars)
            };
            if position < row_chars.len() - 1 {
                insert_beam(position + 1, splits, row_chars)
            };
        }
        _ => {}
    }
}
