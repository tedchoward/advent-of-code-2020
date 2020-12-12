const NORTH: i32 = 0;
const SOUTH: i32 = 180;
const EAST: i32 = 90;
const WEST: i32 = 270;

fn load_input() -> String {
    std::fs::read_to_string("dec12/src/input.txt").expect("Unable to read input file")
}

fn normalize_direction(direction: i32) -> i32 {
    if direction < 0 {
        return 360 + direction;
    }

    if direction >= 360 {
        return direction % 360;
    }

    direction
}

fn puzzle_1() -> i32 {
//     let input = String::from(
//         "F10
// N3
// F7
// R90
// F11",
//     );

    let input = load_input();

    let mut east_west = 0;
    let mut north_south = 0;
    let mut direction = 90;

    for instruction in input.split('\n') {
        let action = &instruction[0..1];
        let value: i32 = instruction[1..].parse().unwrap();

        match action {
            "N" => north_south += value,
            "S" => north_south -= value,
            "E" => east_west += value,
            "W" => east_west -= value,
            "L" => direction -= value,
            "R" => direction += value,
            "F" => {
                direction = normalize_direction(direction);
                match direction {
                    NORTH => north_south += value,
                    SOUTH => north_south -= value,
                    EAST => east_west += value,
                    WEST => east_west -= value,
                    _ => panic!("Unexpected direction: {}", direction),
                }
            }
            _ => panic!("Unknown action: {}", action),
        }
    }

    north_south.abs() + east_west.abs()
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}
