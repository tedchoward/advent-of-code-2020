fn one_iteration(
    partial_sum: usize,
    partial_product: usize,
    slice: &[usize],
    sum: usize,
    iterations_remaining: usize,
) -> Result<usize, String> {
    for i in 0..slice.len() {
        let num = slice[i];
        let partial_sum = partial_sum + num;
        let partial_product = partial_product * num;

        if iterations_remaining == 0 {
            if partial_sum == sum {
                return Ok(partial_product);
            }
        } else {
            match one_iteration(
                partial_sum,
                partial_product,
                &slice[i..],
                sum,
                iterations_remaining - 1,
            ) {
                Ok(product) => return Ok(product),
                Err(_) => {}
            }
        }
    }

    Err(format!("No pairs found that add up to {}", sum))
}

fn find_sum_parts(numbers: Vec<usize>, num_parts: usize, sum: usize) -> Result<usize, String> {
    one_iteration(0, 1, &numbers[0..], sum, num_parts - 1)
}

fn load_input() -> Vec<usize> {
    let input = std::fs::read_to_string("dec01/src/input.txt").expect("Unable to read input file");

    input
        .split('\n')
        .map(|i| i.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

// fn load_sample_input() -> Vec<usize> {
//     vec![
//         1721,
//         979,
//         366,
//         299,
//         675,
//         1456,
//     ]
// }

fn puzzle_1() -> Result<usize, String> {
    let input = load_input();

    find_sum_parts(input, 2, 2020)
}

fn puzzle_2() -> Result<usize, String> {
    let input = load_input();
    find_sum_parts(input, 3, 2020)
}

fn main() {
    match puzzle_1() {
        Ok(result) => println!("Puzzle 1 output: {}", result),
        Err(err) => println!("ERROR: {}", err),
    }

    match puzzle_2() {
        Ok(result) => println!("Puzzle 2 output: {}", result),
        Err(err) => println!("ERROR: {}", err),
    }
}
