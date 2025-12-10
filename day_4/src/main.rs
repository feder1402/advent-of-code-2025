fn main() {
    let map_data = include_str!("../puzzle_input.txt");
    let lines = map_data.lines();

    let mut map: Vec<Vec<char>> = Vec::new();

    for line in lines {
        map.push(line.chars().collect());
    }

    let mut total_paper_rolls = 0;
    let mut new_map = map;
    loop {
        let (accessible_paper_rolls, accessible) = get_accessibility_map(&new_map);
        new_map = accessible;

        for row in new_map.iter() {
            let line = row.iter().collect::<String>();
            println!("{}", line);
        }

        println!("Accessible paper rolls: {}", accessible_paper_rolls);

        if accessible_paper_rolls == 0 { break;}

        total_paper_rolls += accessible_paper_rolls;
    }

    println!("Total paper rolls: {}", total_paper_rolls);
}

fn get_accessibility_map(map: &Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    let mut accessible_paper_rolls = 0;
    let mut accessible: Vec<Vec<char>> = Vec::new();
    for i in 0..map.len() {
        let mut row = map[i].clone();
        for j in 0..map[i].len() {
            if map[i][j] == '@' {
                let neighbors = get_neighbors(i, j, &map);
                if neighbors < 4 {
                    row[j] = 'x';
                    accessible_paper_rolls += 1;
                } else {
                    row[j] = map[i][j];
                }
            }
        }
        accessible.push(row);
    }

    (accessible_paper_rolls, accessible)
}

fn get_neighbors(i: usize, j: usize, map: &Vec<Vec<char>>) -> usize {
    let mut neighbors: usize = 0;
    let row: isize = i as isize;
    let col: isize = j as isize;

    for row_offset in -1..=1 {
        for col_offset in -1..=1 {
            let neighbor_row = row + row_offset;
            let neighbor_col = col + col_offset;
            if neighbor_row >= 0
                && neighbor_row < map.len().cast_signed()
                && neighbor_col >= 0
                && neighbor_col < map[neighbor_row.unsigned_abs()].len().cast_signed()
                && (col_offset != 0 || row_offset != 0)
            {
                if map[neighbor_row.unsigned_abs()][neighbor_col.unsigned_abs()] == '@' {
                    neighbors += 1;
                }
            }
        }
    }

    neighbors
}
