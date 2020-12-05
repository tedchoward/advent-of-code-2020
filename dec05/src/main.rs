fn load_input() -> Vec<String> {
    let input = std::fs::read_to_string("dec05/src/input.txt").expect("Unable to read input file");

    input.split('\n').map(String::from).collect::<Vec<String>>()
}

fn lower_half(slice: &[u32]) -> &[u32] {
    let end = slice.len() / 2;
    &slice[0..end]
}

fn upper_half(slice: &[u32]) -> &[u32] {
    let start = slice.len() / 2;
    &slice[start..]
}

fn find_row(boarding_pass: &str, rows: &[u32]) -> u32 {
    let mut slice = rows;
    for ch in boarding_pass.chars() {
        slice = match ch {
            'F' => lower_half(slice),
            'B' => upper_half(slice),
            _ => panic!("Unexpected row value: {}", ch),
        }
    }

    slice[0]
}

fn find_column(boarding_pass: &str, columns: &[u32]) -> u32 {
    let mut slice = columns;
    for ch in boarding_pass.chars() {
        slice = match ch {
            'L' => lower_half(slice),
            'R' => upper_half(slice),
            _ => panic!("Unexpected column value: {}", ch),
        }
    }

    slice[0]
}

fn get_seat_id(row: u32, column: u32) -> u32 {
    row * 8 + column
}

fn puzzle_1() -> u32 {
    let rows: Vec<u32> = (0..128).collect();
    let columns: Vec<u32> = (0..8).collect();

    // let input = vec![
    //     String::from("FBFBBFFRLR"),
    //     String::from("BFFFBBFRRR"),
    //     String::from("FFFBBBFRRR"),
    //     String::from("BBFFBBFRLL"),
    // ];
    let input = load_input();

    let mut highest_seat_id = 0;

    for boarding_pass in input {
        let row = find_row(&boarding_pass[0..7], &rows);
        print!("Row: {}, ", row);

        let column = find_column(&boarding_pass[7..], &columns);
        print!("Column: {}, ", column);

        let seat_id = get_seat_id(row, column);
        println!("Seat Id: {}", seat_id);

        if seat_id > highest_seat_id {
            highest_seat_id = seat_id
        }
    }

    highest_seat_id
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}
