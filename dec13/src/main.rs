fn load_input() -> String {
    std::fs::read_to_string("dec13/src/input.txt").expect("Unable to read input file")
}

fn calculate_x(product_other_moduli: usize, modulus: usize) -> usize {
    let reduced = product_other_moduli % modulus;
    for i in 1..=usize::MAX {
        if (reduced * i) % modulus == 1 {
            return i;
        }
    }

    panic!(
        "xi not found for Ni={} ni={}",
        product_other_moduli, modulus
    );
}

/// chinese remainder theorem
fn chinese(remainders: &[usize], moduli: &[usize]) -> usize {
    let moduli_product = moduli.iter().fold(1, |a, n| a * n);

    let x: usize = (0..remainders.len())
        .map(|i| {
            let remainder = remainders[i];
            let product_other_moduli = moduli_product / moduli[i];
            let x = calculate_x(product_other_moduli, moduli[i]);
            remainder * product_other_moduli * x
        })
        .sum();

    x % moduli_product
}

fn puzzle_1() -> u32 {
    //     let input = String::from("939
    // 7,13,x,x,59,x,31,19");

    let input = load_input();

    let parts = input.split('\n').collect::<Vec<&str>>();
    let earliest_time: u32 = parts[0].parse().unwrap();

    let mut earliest_bus_time = u32::MAX;
    let mut earliest_bus_id = u32::MAX;

    for bus_id in parts[1]
        .split(',')
        .filter(|bus_id| *bus_id != "x")
        .map(|bus_id| bus_id.parse::<u32>().unwrap())
    {
        let bus_time = ((earliest_time / bus_id) + 1) * bus_id;

        if bus_time < earliest_bus_time {
            earliest_bus_time = bus_time;
            earliest_bus_id = bus_id;
        }
    }

    let wait_time = earliest_bus_time - earliest_time;
    earliest_bus_id * wait_time
}

fn puzzle_2() -> usize {
    //             let input = String::from("939
    // 7,13,x,x,59,x,31,19");

    let input = load_input();
    let bus_ids = input.split('\n').nth(1).unwrap();

    let pairs = bus_ids
        .split(',')
        .enumerate()
        .filter_map(|p| {
            if p.1 == "x" {
                None
            } else {
                Some((p.0, p.1.parse::<usize>().unwrap()))
            }
        })
        .collect::<Vec<(usize, usize)>>();

    let remainders = pairs
        .iter()
        .map(|p| p.1 - (p.0 % p.1))
        .collect::<Vec<usize>>();
    let moduli = pairs.iter().map(|p| p.1).collect::<Vec<usize>>();

    chinese(&remainders, &moduli)
}

fn main() {
    let start = std::time::Instant::now();
    let result = puzzle_1();
    let elapsed = start.elapsed().as_secs();
    println!("Puzzle 1 output: {}", result);
    println!("Operation took {} seconds", elapsed);
    let start = std::time::Instant::now();
    let result = puzzle_2();
    let elapsed = start.elapsed().as_secs();
    println!("Puzzle 2 output: {}", result);
    println!("Operation took {} seconds", elapsed);
}
