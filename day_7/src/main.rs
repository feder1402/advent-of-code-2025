fn main() {
    // Read the input
    let input = include_str!("../sample_input.txt");
    let mut lines = input.lines();

    let mut line = lines.next();
    let mut splits: usize = 0;
    let mut beams: Vec<usize> = Vec::new();
    let mut diagram: Vec<Vec<char>> = Vec::new();
    loop {
        if line.is_none() {
            break;
        };
        let mut row_chars = line.unwrap().chars().collect::<Vec<char>>();

        for beam in beams {
            (splits, row_chars) = insert_beam(beam, splits, row_chars);
        }

        beams = row_chars
            .iter()
            .enumerate()
            .filter(|item| ['S', '|'].contains(item.1))
            .map(|item| item.0)
            .collect();

        diagram.push(row_chars);

        line = lines.next();
    }

    println!("Diagram:");
    for r in &diagram {
        let row: String = r.iter().collect();
        println!("{}", row);
    }
    println!("Splits: {splits}");

    let initial_beam_position = diagram[0].iter().position(|x| *x == 'S').unwrap();
    let timelines = count_timelines(initial_beam_position, &diagram, 0, diagram.len() - 1);

    println!("Timelines {timelines}");
}

fn count_timelines(position: usize, diagram: &Vec<Vec<char>>, depth: usize, max_depth: usize) -> usize {
    if depth >= max_depth {
        return 1;
    };

    let row_chars = &diagram[depth];
    let mut timelines = 0;
    if row_chars[position] == '^' {
        if position > 0 {
            timelines += count_timelines(position - 1, diagram, depth + 1, max_depth);
        };
        if position < row_chars.len() - 2 {
            timelines += count_timelines(position + 1, diagram, depth + 1, max_depth);
        };
    } else {
        timelines = count_timelines(position, diagram, depth + 1, max_depth);
    }
    timelines
}

fn insert_beam(position: usize, mut splits: usize, mut row_chars: Vec<char>) -> (usize, Vec<char>) {
    match row_chars[position] {
        '.' => row_chars[position] = '|',
        '^' => {
            splits += 1;
            if position > 0 {
                (splits, row_chars) = insert_beam(position - 1, splits, row_chars)
            };
            if position < row_chars.len() - 1 {
                (splits, row_chars) = insert_beam(position + 1, splits, row_chars)
            };
        }
        _ => {}
    }

    (splits, row_chars)
}

// fn count_timelines(line: usize, position: usize, diagram: &Vec<Vec<char>>, path: &Vec<usize>, mut path_set: &mut BTreeSet<String>) -> usize {
//     let mut new_timelines: usize = 0;
//     let mut new_path = path.clone();
//     let c = diagram[line][position];
//     if ['|', 'S'].contains(&c) {
//         new_path.push(position);
//         if line == diagram.len() - 1 {
//             let path_str: String = new_path.iter().map(|x|x.to_string()).collect::<Vec<_>>().join(" ");
//             println!("Path: {}", path_str);
//             if path_set.contains(&path_str) {println!("Found duplicate!: {:?}", new_path)}
//             path_set.insert(path_str);
//             return 1;
//         }
//         if position > 0 { new_timelines += count_timelines(line + 1, position - 1, diagram, &new_path, &mut path_set) };
//         new_timelines += count_timelines(line + 1, position, diagram, &new_path, &mut path_set);
//         if position < diagram[line].len() - 2 { new_timelines += count_timelines(line + 1, position + 1, diagram, &new_path, &mut path_set) };
//     };
//
//     new_timelines
// }
