pub fn run(input: &str) {
    let data: Vec<Vec<&str>> = input.lines()
        .map(|line| 
            line.split("")
                .filter(|&c| !c.is_empty())
                .collect())
        .collect();
    let mut word_count = 0;
    let mut x_word_count = 0;
    for i in 0..data.len() {
        for j in 0..data[i].len() {
            if data[i][j] == "X" {
                word_count += find_xmas(&data, i, j);
            } else if data[i][j] == "A" && find_x_mas(&data, i, j) {
                x_word_count += 1;
            }
        }
    }
    
    println!("{}", word_count);
    println!("{}", x_word_count)
}

/// Finds the word XMAS in all directions starting from the position of X
pub fn find_xmas(data: &Vec<Vec<&str>>, row: usize, col: usize) -> i32 {
    let directions = vec![
        // (row, col)
        (-1, 0), // up
        (-1, 1), // up-right
        (0, 1),  // right
        (1, 1),  // down-right
        (1, 0),  // down
        (1, -1), // down-left
        (0, -1), // left
        (-1, -1) // up-left
    ];
    let height = data.len();
    let width = data.first().unwrap().len();
    
    let mut word_count = 0;
    'outer: for (row_step, col_step) in directions {
        let mut row_curr = row;
        let mut col_curr = col;
        for letter in vec!["X", "M", "A", "S"] {
            if (0..height).contains(&row_curr) 
                && (0..width).contains(&col_curr) 
                && data[row_curr][col_curr].contains(letter) {
                row_curr = (row_curr as i32 + row_step) as usize;
                col_curr = (col_curr as i32 + col_step) as usize;
            } else {
                continue 'outer;
            }
        }
        word_count += 1
    }
    word_count
}

/// Find X-MAS:
/// ```text
/// M.S
/// .A.
/// M.S
/// ```
pub fn find_x_mas(data: &Vec<Vec<&str>>, row: usize, col: usize) -> bool {
    let height = data.len();
    let width = data.first().unwrap().len();
    
    if row > 0 && col > 0 
        && (row-1..=row+1).all(|i| (0..height).contains(&i))
        && (col-1..=col+1).all(|j| (0..width).contains(&j)){
        let diag1 = vec![data[row-1][col-1], data[row][col], data[row+1][col+1]];
        let diag2 = vec![data[row+1][col-1], data[row][col], data[row-1][col+1]];

        
        if vec!["M", "A", "S"].iter().all(|c| diag1.contains(&c) && diag2.contains(&c)) {
            return true;
        }
    }
    false
}