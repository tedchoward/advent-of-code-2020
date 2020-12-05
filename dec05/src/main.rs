fn load_input() -> Vec<String> {
    let input = std::fs::read_to_string("dec05/src/input.txt").expect("Unable to read input file");

    input.split('\n').map(String::from).collect::<Vec<String>>()
}

fn both_puzzles() -> (u32, u32) {
    let seat_ids = {
        let mut seats = load_input()
            .iter()
            .map(|boarding_pass| {
                u32::from_str_radix(
                    &boarding_pass
                        .replace("F", "0")
                        .replace("B", "1")
                        .replace("L", "0")
                        .replace("R", "1"),
                    2,
                )
                .unwrap()
            })
            .collect::<Vec<u32>>();

        seats.sort();
        seats
    };

    let highest_seat_id = seat_ids.iter().max().unwrap();
    let available_seat = seat_ids
        .iter()
        .enumerate()
        .filter_map(|(index, seat_id)| {
            if index != 0 && seat_ids[index - 1] != *seat_id - 1 {
                Some(*seat_id - 1)
            } else {
                None
            }
        })
        .nth(0)
        .unwrap();

    (*highest_seat_id, available_seat)
}

fn main() {
    let (highest_seat_id, available_seat) = both_puzzles();
    println!("Puzzle 1 output: {}", highest_seat_id);
    println!("Puzzle 2 output: {}", available_seat);
}
