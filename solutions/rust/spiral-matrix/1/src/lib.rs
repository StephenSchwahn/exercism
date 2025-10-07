const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // Initialize the matrix
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    if size == 0 {
        return matrix;
    }

    matrix[0][0] = 1;

    let mut directions = DIRECTIONS.iter().cycle();
    let mut count = 0;
    let mut current_direction = directions.next().unwrap();
    let mut current = (0isize, 0isize);

    while count < size * size {
        matrix[current.0 as usize][current.1 as usize] = count + 1;
        count += 1;

        let prev = current;
        current = (
            (current.0 + current_direction.0),
            (current.1 + current_direction.1),
        );
        // Check for oob or occupied
        if current.0 >= size as isize
            || current.0 < 0
            || current.1 >= size as isize
            || current.1 < 0
            || matrix[current.0 as usize][current.1 as usize] != 0
        {
            current_direction = directions.next().unwrap();
            current = (
                (prev.0 + current_direction.0),
                (prev.1 + current_direction.1),
            );
        }
    }

    matrix
}
