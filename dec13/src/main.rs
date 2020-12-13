fn load_input() -> String {
    std::fs::read_to_string("dec13/src/input.txt").expect("Unable to read input file")
}

fn puzzle_1() -> u32 {
//     let input = String::from("939
// 7,13,x,x,59,x,31,19");

    let input = load_input();

    let parts = input.split('\n').collect::<Vec<&str>>();
    let earliest_time: u32 = parts[0].parse().unwrap();

    let mut earliest_bus_time = u32::MAX;
    let mut earliest_bus_id = u32::MAX;

    for bus_id in parts[1].split(',').filter(|bus_id| *bus_id != "x").map(|bus_id| bus_id.parse::<u32>().unwrap()) {
        let bus_time = ((earliest_time / bus_id) + 1) * bus_id;

        if bus_time < earliest_bus_time {
            earliest_bus_time = bus_time;
            earliest_bus_id = bus_id;
        }
    }

    let wait_time = earliest_bus_time - earliest_time;
    earliest_bus_id * wait_time
}

fn main() {
    let result = puzzle_1();
    println!("Puzzle 1 output: {}", result);
}
