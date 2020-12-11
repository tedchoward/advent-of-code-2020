fn load_input() -> Vec<Vec<char>> {
    std::fs::read_to_string("dec11/src/input.txt")
        .expect("Unable to read input file")
        .split('\n')
        .map(|s| s.chars().collect())
        .collect::<Vec<Vec<char>>>()
}

fn should_become_occupied(seats: &Vec<Vec<char>>, column: usize, row: usize) -> bool {
    let num_rows = seats.len();
    let num_cols = seats[row].len();

    if seats[row][column] == 'L' {
        let left_occupied = column > 0 && seats[row][column - 1] == '#';
        let right_occupied = column + 1 < num_cols && seats[row][column + 1] == '#';
        let top_occupied = row > 0 && seats[row - 1][column] == '#';
        let bottom_occupied = row + 1 < num_rows && seats[row + 1][column] == '#';

        let top_left_occupied = row > 0 && column > 0 && seats[row - 1][column - 1] == '#';
        let top_right_occupied =
            row > 0 && column + 1 < num_cols && seats[row - 1][column + 1] == '#';

        let bottom_left_occupied =
            row + 1 < num_rows && column > 0 && seats[row + 1][column - 1] == '#';
        let bottom_right_occupied =
            row + 1 < num_rows && column + 1 < num_cols && seats[row + 1][column + 1] == '#';

        return !top_left_occupied
            && !top_occupied
            && !top_right_occupied
            && !left_occupied
            && !right_occupied
            && !bottom_left_occupied
            && !bottom_occupied
            && !bottom_right_occupied;
    }

    false
}

fn new_should_become_occupied(seats: &Vec<Vec<char>>, column: usize, row: usize) -> bool {
    let num_rows = seats.len();
    let num_cols = seats[row].len();

    if seats[row][column] == 'L' {
        // left
        let mut dx = 1;
        while dx <= column {
            if seats[row][column - dx] == '#' {
                return false;
            } else if seats[row][column - dx] == 'L' {
                break;
            }

            dx += 1;
        }

        // right
        let mut dx = 1;
        while (column + dx) < num_cols {
            if seats[row][column + dx] == '#' {
                return false;
            } else if seats[row][column + dx] == 'L' {
                break;
            }

            dx += 1;
        }

        // top
        let mut dy = 1;
        while dy <= row {
            if seats[row - dy][column] == '#' {
                return false;
            } else if seats[row - dy][column] == 'L' {
                break;
            }

            dy += 1;
        }

        // bottom
        let mut dy = 1;
        while row + dy < num_rows {
            if seats[row + dy][column] == '#' {
                return false;
            } else if seats[row + dy][column] == 'L' {
                break;
            }

            dy += 1;
        }

        // top-left
        let mut dy = 1;
        let mut dx = 1;
        while dy <= row && dx <= column {
            if seats[row - dy][column - dx] == '#' {
                return false;
            } else if seats[row - dy][column - dx] == 'L' {
                break;
            }

            dy += 1;
            dx += 1;
        }

        // top-right
        let mut dy = 1;
        let mut dx = 1;
        while dy <= row && (column + dx) < num_cols {
            if seats[row - dy][column + dx] == '#' {
                return false;
            } else if seats[row - dy][column + dx] == 'L' {
                break;
            }

            dy += 1;
            dx += 1;
        }

        // bottom-left
        let mut dy = 1;
        let mut dx = 1;
        while row + dy < num_rows && dx <= column {
            if seats[row + dy][column - dx] == '#' {
                return false;
            } else if seats[row + dy][column - dx] == 'L' {
                break;
            }

            dy += 1;
            dx += 1;
        }

        // bottom-left
        let mut dy = 1;
        let mut dx = 1;
        while row + dy < num_rows && (column + dx) < num_cols {
            if seats[row + dy][column + dx] == '#' {
                return false;
            } else if seats[row + dy][column + dx] == 'L' {
                break;
            }

            dy += 1;
            dx += 1;
        }

        return true;
    }

    false
}

fn should_become_empty(seats: &Vec<Vec<char>>, column: usize, row: usize) -> bool {
    let num_rows = seats.len();
    let num_cols = seats[row].len();

    if seats[row][column] == '#' {
        let mut occupied_count = 0;
        for r in -1..=1 {
            for c in -1..=1 {
                if !(r == 0 && c == 0) {
                    let y = (row as i32) + r;
                    let x = (column as i32) + c;
                    if x >= 0 && y >= 0 && (y as usize) < num_rows && (x as usize) < num_cols {
                        let seat = seats[y as usize][x as usize];
                        if seat == '#' {
                            occupied_count += 1;
                            if occupied_count >= 4 {
                                return true;
                            }
                        }
                    }
                }
            }
        }
    }
    false
}

fn new_should_become_empty(seats: &Vec<Vec<char>>, column: usize, row: usize) -> bool {
    let occupied_threshold = 5;
    let num_rows = seats.len();
    let num_cols = seats[row].len();

    if seats[row][column] == '#' {
        let mut occupied_count = 0;
        // left
        let mut dx = 1;
        while dx <= column {
            if seats[row][column - dx] == '#' {
                occupied_count += 1;
                break;
            } else if seats[row][column - dx] == 'L' {
                break;
            }

            dx += 1;
        }

        if occupied_count >= occupied_threshold {
            return true;
        }

        // right
        let mut dx = 1;
        while (column + dx) < num_cols {
            if seats[row][column + dx] == '#' {
                occupied_count += 1;
                break;
            } else if seats[row][column + dx] == 'L' {
                break;
            }

            dx += 1;
        }

        if occupied_count >= occupied_threshold {
            return true;
        }

        // top
        let mut dy = 1;
        while dy <= row {
            if seats[row - dy][column] == '#' {
                occupied_count += 1;
                break;
            } else if seats[row - dy][column] == 'L' {
                break;
            }

            dy += 1;
        }

        if occupied_count >= occupied_threshold {
            return true;
        }

        // bottom
        let mut dy = 1;
        while row + dy < num_rows {
            if seats[row + dy][column] == '#' {
                occupied_count += 1;
                break;
            } else if seats[row + dy][column] == 'L' {
                break;
            }

            dy += 1;
        }

        if occupied_count >= occupied_threshold {
            return true;
        }

        // top-left
        let mut dy = 1;
        let mut dx = 1;
        while dy <= row && dx <= column {
            if seats[row - dy][column - dx] == '#' {
                occupied_count += 1;
                break;
            } else if seats[row - dy][column - dx] == 'L' {
                break;
            }

            dy += 1;
            dx += 1;
        }

        if occupied_count >= occupied_threshold {
            return true;
        }

        // top-right
        let mut dy = 1;
        let mut dx = 1;
        while dy <= row && (column + dx) < num_cols {
            if seats[row - dy][column + dx] == '#' {
                occupied_count += 1;
                break;
            } else if seats[row - dy][column + dx] == 'L' {
                break;
            }

            dy += 1;
            dx += 1;
        }

        if occupied_count >= occupied_threshold {
            return true;
        }

        // bottom-left
        let mut dy = 1;
        let mut dx = 1;
        while row + dy < num_rows && dx <= column {
            if seats[row + dy][column - dx] == '#' {
                occupied_count += 1;
                break;
            } else if seats[row + dy][column - dx] == 'L' {
                break;
            }

            dy += 1;
            dx += 1;
        }

        if occupied_count >= occupied_threshold {
            return true;
        }

        // bottom-left
        let mut dy = 1;
        let mut dx = 1;
        while row + dy < num_rows && (column + dx) < num_cols {
            if seats[row + dy][column + dx] == '#' {
                occupied_count += 1;
                break;
            } else if seats[row + dy][column + dx] == 'L' {
                break;
            }

            dy += 1;
            dx += 1;
        }

        if occupied_count >= occupied_threshold {
            return true;
        }
    }

    false
}

// fn print_grid(seats: &Vec<Vec<char>>) {
//     for row in seats {
//         for seat in row {
//             print!("{}", seat);
//         }

//         println!("");
//     }

//     println!("");
// }

fn puzzle_1() -> u32 {
    let mut input = load_input();
    // print_grid(&input);
    let mut count: u32;

    loop {
        let mut state_changed = false;
        let mut clone = input.clone();
        count = 0;

        for (row, seats) in input.iter().enumerate() {
            for (col, seat) in seats.iter().enumerate() {
                if *seat == 'L' && should_become_occupied(&input, col, row) {
                    clone[row][col] = '#';
                    state_changed = true;
                } else if *seat == '#' {
                    if should_become_empty(&input, col, row) {
                        clone[row][col] = 'L';
                        state_changed = true;
                    }
                    count += 1;
                }
            }
        }

        input = clone;

        // print_grid(&input);

        if !state_changed {
            break;
        }
    }

    count
}

fn puzzle_2() -> u32 {
    let mut input = load_input();
    // print_grid(&input);
    let mut count: u32;


    loop {
        let mut state_changed = false;
        let mut clone = input.clone();
        count = 0;

        for (row, seats) in input.iter().enumerate() {
            for (col, seat) in seats.iter().enumerate() {
                if *seat == 'L' && new_should_become_occupied(&input, col, row) {
                    clone[row][col] = '#';
                    state_changed = true;
                } else if *seat == '#' {
                    if new_should_become_empty(&input, col, row) {
                        clone[row][col] = 'L';
                        state_changed = true;
                    }
                    count += 1;
                }
            }
        }

        input = clone;

        // print_grid(&input);

        if !state_changed {
            break;
        }
    }

    count
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2 output: {}", result);
}
