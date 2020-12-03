struct Slope {
    right: usize,
    down: usize,
}

fn count_trees(input: &Vec<String>, slope: &Slope) -> u64 {
    let rows = input.len();
    let row_size = input[0].len();

    let mut tree_count = 0;

    let mut x = 0;
    for y in (slope.down..rows).step_by(slope.down) {
        x += slope.right;

        let char = input[y].chars().nth(x % row_size).unwrap();
        if char == '#' {
            tree_count += 1;
        }
    }

    println!("Tree count: {}", tree_count);

    tree_count
}

fn load_input() -> Vec<String> {
    let input = std::fs::read_to_string("dec03/src/input.txt").expect("Unable to read input file");

    input.split('\n').map(String::from).collect::<Vec<String>>()
}

fn puzzle_1() -> u64 {
    // let input = vec![
    //     String::from("..##......."),
    //     String::from("#...#...#.."),
    //     String::from(".#....#..#."),
    //     String::from("..#.#...#.#"),
    //     String::from(".#...##..#."),
    //     String::from("..#.##....."),
    //     String::from(".#.#.#....#"),
    //     String::from(".#........#"),
    //     String::from("#.##...#..."),
    //     String::from("#...##....#"),
    //     String::from(".#..#...#.#"),
    // ];

    let input = load_input();
    let slope = Slope { right: 3, down: 1 };

    count_trees(&input, &slope)
}

fn puzzle_2() -> u64 {
    // let input = vec![
    //     String::from("..##......."),
    //     String::from("#...#...#.."),
    //     String::from(".#....#..#."),
    //     String::from("..#.#...#.#"),
    //     String::from(".#...##..#."),
    //     String::from("..#.##....."),
    //     String::from(".#.#.#....#"),
    //     String::from(".#........#"),
    //     String::from("#.##...#..."),
    //     String::from("#...##....#"),
    //     String::from(".#..#...#.#"),
    // ];

    let input = load_input();
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 3, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    slopes
        .iter()
        .map(|slope| count_trees(&input, slope))
        .product()
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);

    let result = puzzle_2();
    println!("Puzzle 2 output: {}", result);
}
