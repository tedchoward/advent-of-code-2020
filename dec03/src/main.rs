fn load_input() -> Vec<String> {
    let input = std::fs::read_to_string("dec03/src/input.txt").expect("Unable to read input file");

    input.split('\n').map(String::from).collect::<Vec<String>>()
}

fn puzzle_1() -> u32 {
    // let input = vec![
    //     "..##.......",
    //     "#...#...#..",
    //     ".#....#..#.",
    //     "..#.#...#.#",
    //     ".#...##..#.",
    //     "..#.##.....",
    //     ".#.#.#....#",
    //     ".#........#",
    //     "#.##...#...",
    //     "#...##....#",
    //     ".#..#...#.#",
    // ];

    let input = load_input();

    let rows = input.len();
    let row_size = input[0].len();

    let mut tree_count = 0;

    let mut x = 0;
    for y in 1..rows {
        x += 3;

        let char = input[y].chars().nth(x % row_size).unwrap();
        if char == '#' {
            tree_count += 1;
        }
    }

    return tree_count;
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}
