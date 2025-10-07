pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();
    if input.len() == 0 || input[0].len() == 0 {
        return saddle_points;
    }

    for row_idx in 0..input.len() {
        let max_value = input[row_idx].iter().max().unwrap();
        let column_candidates: Vec<usize> = input[row_idx]
            .iter()
            .enumerate()
            .filter(|(_, elem)| *elem == max_value)
            .map(|(col_idx, _)| col_idx)
            .collect();

        'outer: for column_idx in column_candidates {
            for row_idx in 0..input.len() {
                let value_at_pt = &input[row_idx][column_idx];
                if max_value > value_at_pt {
                    continue 'outer;
                } 
            }
            saddle_points.push((row_idx, column_idx))
        }
    }
    saddle_points
}
