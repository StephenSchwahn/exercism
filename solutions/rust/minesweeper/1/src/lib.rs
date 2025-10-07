fn get_char_at_index<'a>(minefield: &[&'a str], row: i32, col: i32) -> Option<&'a str> {
    if row >= 0 && row < minefield.len() as i32 && col >= 0 && col < minefield[0].len() as i32 {
        minefield[row as usize].get((col as usize)..(col as usize) + 1)
    } else {
        None
    }
}

fn count_adjacent(minefield: &[&str], row: i32, col: i32) -> u32 {
    let mut count = 0u32;
    let min_row = i32::max(row - 1, 0);
    let max_row = i32::min(row + 1, minefield.len() as i32);
    let min_col = i32::max(col - 1, 0);
    let max_col = i32::min(col + 1, minefield[0].len() as i32);

    for r in min_row..=max_row {
        for c in min_col..=max_col {
            if (r, c) != (row, col) && get_char_at_index(minefield, r, c) == Some("*")
            {
                println!("Char at ({}, {}): {:#?}", r, c, get_char_at_index(minefield, r, c));
                count += 1
            }
        }
    }
    count
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // todo!("\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n");
    minefield
        .iter()
        .enumerate()
        .map(|(row, &s)| {
            s.chars()
                .enumerate()
                .map(|(col, c)| match c {
                    '*' => '*',
                    _ => {
                        let count = count_adjacent(minefield, row as i32, col as i32);
                        if count > 0 {
                            char::from_digit(count, 10).expect("number wasn't a number")
                        } else {
                            ' '
                        }
                    }
                })
                .collect::<String>()
        })
        .collect()
}
