pub struct PascalsTriangle(Vec<Vec<u32>>);

fn compute_triangle(row: u32) -> Vec<Vec<u32>> {
    if row == 0 {
        vec![]
    } else if row == 1 {
        vec![vec![1]]
    } else {
        let mut row_vec: Vec<u32> = Vec::with_capacity(row as usize + 3);

        let mut prev_triangle = compute_triangle(row - 1);

        // Append 0's to the start and end for computation purpose
        let mut last_row = prev_triangle[row as usize - 2].clone();
        last_row.push(0);
        last_row.insert(0, 0);

        // Create a new row but adding up the elements above
        for i in 0usize..(row as usize) {
            row_vec.push(last_row[i] + last_row[i + 1]);
        }

        prev_triangle.push(row_vec);
        prev_triangle
    }
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(compute_triangle(row_count))
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
