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

fn rotate_point(point: &Point, origin: &Point, angle: i32) -> Point {
    let p = Point { x: point.x - origin.x, y: point.y - origin.y };

    match angle {
        90 => Point { x: p.y + origin.x, y: -p.x + origin.y },
        180 => Point { x: -p.x + origin.x, y: -p.y + origin.y },
        270 => Point { x: -p.y + origin.x, y: p.x + origin.y },
        360 => Point { x: p.x + origin.x, y: p.x + origin.y },
        _ => panic!("Unexpected angle: {}", angle),
    }
}

fn puzzle_1() -> i32 {
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

struct Point {
    x: i32,
    y: i32,
}

fn puzzle_2() -> i32 {
    let input = load_input();

    let mut ship_position = Point { x: 0, y: 0 };
    let mut waypoint = Point { x: 10, y: 1 };

    for instruction in input.split('\n') {
        let action = &instruction[0..1];
        let value: i32 = instruction[1..].parse().unwrap();

        match action {
            "N" => waypoint.y += value,
            "S" => waypoint.y -= value,
            "E" => waypoint.x += value,
            "W" => waypoint.x -= value,
            "L" => {
                let rotation = normalize_direction(-value);
                waypoint = rotate_point(&waypoint, &ship_position, rotation);
            },
            "R" => {
                let rotation = normalize_direction(value);
                waypoint = rotate_point(&waypoint, &ship_position, rotation);
            }
            "F" => {
                let waypoint_relative = Point { x: waypoint.x - ship_position.x, y: waypoint.y - ship_position.y };
                for _ in 0..value {
                    ship_position.x = waypoint.x;
                    ship_position.y = waypoint.y;
                    waypoint.x += waypoint_relative.x;
                    waypoint.y += waypoint_relative.y;
                }
            }
            _ => panic!("Unknown action: {}", action),
        }
    }

    ship_position.x.abs() + ship_position.y.abs()
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2 output: {}", result);
}
