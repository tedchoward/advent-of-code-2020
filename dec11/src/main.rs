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

fn print_grid(seats: &Vec<Vec<char>>) {
    for row in seats {
        for seat in row {
            print!("{}", seat);
        }

        println!("");
    }

    println!("");
}

fn puzzle_1() -> u32 {

//     let mut input = String::from(
//         "L.LL.LL.LL
// LLLLLLL.LL
// L.L.L..L..
// LLLL.LL.LL
// L.LL.LL.LL
// L.LLLLL.LL
// ..L.L.....
// LLLLLLLLLL
// L.LLLLLL.L
// L.LLLLL.LL",
//     )
//     .split('\n')
//     .map(|s| s.chars().collect())
//     .collect::<Vec<Vec<char>>>();

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

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}
