static NEIGBOURHOOD_OFFSETS: &'static [(i32, i32)] = &[
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    let mut matrix = Vec::new();

    for line in minefield.iter() {
        let mut line_values = Vec::new();
        for char in line.chars() {
            line_values.push(String::from(char));
        }
        matrix.push(line_values);
    }

    for row in 0..matrix.len() {
        for col in 0..matrix[row].len() {
            let position = (row, col);
            //println!("Checking {},{}",i,j);
            let amount = check_amount(position, &matrix);
            if amount == -1 || amount == 0 {
                // it's a miner in this position
                continue;
            }
            let amount_str = amount.to_string();
            matrix[row][col] = amount_str
        }
    }
    //println!("Matrix after");
    for i in 0..matrix.len() {
        //println!("{:?}", matrix[i]);
        let mut row = "".to_string();
        for j in 0..matrix[i].len() {
            row.push_str(&matrix[i][j]);
        }
        result.push(row)
    }
    result
}

fn check_amount(position: (usize, usize), matrix: &Vec<Vec<String>>) -> i32 {
    // check if in the position there is miner:
    let value_in_position = &matrix[position.0][position.1];
    if value_in_position == "*" {
        return -1;
    }
    let mut miner_founds = 0;
    // println!("{:?}",matrix);
    // println!("{:?}", position);
    for direction in NEIGBOURHOOD_OFFSETS.iter() {
        //println!("comparing direction: {:?}",direction);
        let row = position.0 as i32 + direction.0;
        let col = position.1 as i32 + direction.1;
        //println!("Check direction {},{}",x,y);
        if  row < 0 || row as usize > matrix.len() -1 {
            continue;
        }
        if col < 0 || col as usize > matrix[0].len() -1{
            continue;
        }
        //println!("Accessing index {},{}", row, col);
        let position = &matrix[row as usize][col as usize];
        if position == "*" {
            miner_founds += 1
        }
    }
    miner_founds
}
