
fn find_sum_parts(numbers: Vec<usize>, sum: usize) -> Result<(usize, usize), String> {

    for i in 0..numbers.len() {
        let num1 = numbers[i];
        let slice = &numbers[i..];
        for num2 in slice {
            if num1 + num2 == sum {
                return Ok((num1, *num2))
            }
        }
    }

    Err(format!("No pairs found that add up to {}", sum))
}

fn puzzle_1() -> Result<usize, String> {
    let input = std::fs::read_to_string("dec01/src/input.txt").expect("Unable to read input file");

    let input = input.split('\n');

    let input = input.map(|i| i.parse::<usize>().unwrap()).collect::<Vec<usize>>();

    // let input = vec![
    //     1721,
    //     979,
    //     366,
    //     299,
    //     675,
    //     1456,
    // ];

    let (x, y) = find_sum_parts(input, 2020)?;
    Ok(x * y)
}

fn main() {
    match puzzle_1() {
        Ok(result) => println!("{}", result),
        Err(err) => println!("ERROR: {}", err),
    }
}
