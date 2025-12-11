fn main() {
    // Read the input
    let input = include_str!("../sample_input.txt");
    let mut lines = input.lines();

    let mut line = lines.next();
    let mut splits: usize = 0;
    let mut beams: Vec<usize> = Vec::new();
    loop {
        if line.is_none() {
            break;
        };
        let mut row_chars = line.unwrap().chars().collect::<Vec<char>>();

        for beam in beams {
            (splits, row_chars) = insert_beam(beam, splits, row_chars);
        }

        let row: String = row_chars.iter().collect();
        println!("{}", row);

        beams = row_chars
            .iter()
            .enumerate()
            .filter(|item| ['S', '|'].contains(item.1))
            .map(|item| item.0)
            .collect();

        line = lines.next();
    }

    println!("Splits: {splits}")
}

fn insert_beam(position: usize, mut splits: usize, mut row_chars: Vec<char>) -> (usize, Vec<char>) {
    match row_chars[position] {
        '.' => row_chars[position] = '|',
        '^' => {
            splits += 1;
            if position > 0 {(splits, row_chars) = insert_beam(position -1, splits, row_chars)};
            if position < row_chars.len() - 2 {(splits, row_chars) = insert_beam(position + 1, splits, row_chars)};
        },
        _ => {}
    }

    (splits, row_chars)
}
