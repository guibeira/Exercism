use std::collections::HashMap;

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut matrix: Vec<Vec<String>> = garden
        .iter()
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect();
    let mut directions: HashMap<&str, (i32, i32)> = HashMap::from([
        ("left", (-1, 0)),
        ("top", (0, 1)),
        ("right", (1, 0)),
        ("bottom", (0, -1)),
        ("dtl", (-1, 1)),
        ("dtr", (1, 1)),
        ("dbl", (-1, -1)),
        ("dbr", (1, -1)),
    ]);

    let y_size = matrix.len() as i32;
    let x_size = { if y_size > 0 { matrix[0].len() } else { 0 } } as i32;

    for row_index in 0..matrix.len() {
        for collunm_index in 0..matrix[row_index].len() {
            if matrix[row_index][collunm_index] == "*" {
                continue;
            }
            let mut amount = 0;
            for (_direction, (x, y)) in directions.iter() {
                let x_dir = collunm_index as i32 + x;
                let y_dir = row_index as i32 + y;

                if x_dir < 0 || x_dir >= x_size {
                    continue;
                }
                if y_dir < 0 || y_dir >= y_size {
                    continue;
                }

                if matrix[y_dir as usize][x_dir as usize] == "*" {
                    amount += 1;
                }
            }
            if amount != 0 {
                matrix[row_index][collunm_index] = amount.to_string();
            }
        }
    }
    matrix.into_iter().map(|row| row.join("")).collect()
}
