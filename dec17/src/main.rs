fn load_input() -> String {
    std::fs::read_to_string("dec17/src/input.txt").expect("Unable to read input file")
}

// fn print_plane(plane: &Vec<Vec<bool>>) {
//     for y in 0..plane.len() {
//         let row = &plane[y];
//         for x in 0..row.len() {
//             print!("{}", if row[x] { "#" } else { "." });
//         }

//         println!("");
//     }
// }

// fn print_universe(universe: &Vec<Vec<Vec<bool>>>) {
//     for z in 0..universe.len() {
//         println!("z={}", z);
//         print_plane(&universe[z]);
//         println!("\n");
//     }
// }

fn execute_cycle(pocket_universe: &Vec<Vec<Vec<bool>>>) -> Vec<Vec<Vec<bool>>> {
    // insert and append new planes in the universe
    let mut pocket_universe = pocket_universe.clone();
    pocket_universe.insert(
        0,
        vec![vec![false; pocket_universe[0][0].len()]; pocket_universe[0].len()],
    );
    pocket_universe.push(vec![
        vec![false; pocket_universe[0][0].len()];
        pocket_universe[0].len()
    ]);
    for plane in &mut pocket_universe {
        // insert and append new rows in each plane
        plane.insert(0, vec![false; plane[0].len()]);
        plane.push(vec![false; plane[0].len()]);
        for row in plane {
            // insert and append new columns in each row
            row.insert(0, false);
            row.push(false);
        }
    }

    let universe_len = pocket_universe.len();
    let mut new_universe: Vec<Vec<Vec<bool>>> = Vec::new();
    for z in 0..universe_len {
        let plane = &pocket_universe[z];
        let plane_len = plane.len();
        let mut new_plane: Vec<Vec<bool>> = Vec::new();

        for y in 0..plane_len {
            let row = &plane[y];
            let row_len = row.len();
            let mut new_row: Vec<bool> = Vec::new();

            for x in 0..row_len {
                let column = row[x];
                let mut active_count = 0;

                for dz in -1..=1 {
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if !(dz == 0 && dy == 0 && dx == 0) {
                                let zp = z as i128 + dz;
                                let yp = y as i128 + dy;
                                let xp = x as i128 + dx;
                                if zp > 0
                                    && zp < (universe_len as i128)
                                    && yp > 0
                                    && yp < (plane_len as i128)
                                    && xp > 0
                                    && xp < (row_len as i128)
                                {
                                    if pocket_universe[zp as usize][yp as usize][xp as usize] {
                                        active_count += 1;
                                    }
                                }
                            }
                        }
                    }
                }

                let new_value = if column {
                    if active_count == 2 || active_count == 3 {
                        true
                    } else {
                        false
                    }
                } else {
                    if active_count == 3 {
                        true
                    } else {
                        false
                    }
                };

                new_row.push(new_value);
            }

            new_plane.push(new_row);
        }

        new_universe.push(new_plane);
    }

    new_universe
}

fn execute_hyper_cycle(
    pocket_hyper_universe: &Vec<Vec<Vec<Vec<bool>>>>,
) -> Vec<Vec<Vec<Vec<bool>>>> {
    // insert and append new planes in the universe
    let mut pocket_hyper_universe = pocket_hyper_universe.clone();
    pocket_hyper_universe.insert(
        0,
        vec![
            vec![
                vec![false; pocket_hyper_universe[0][0][0].len()];
                pocket_hyper_universe[0][0].len()
            ];
            pocket_hyper_universe[0].len()
        ],
    );
    pocket_hyper_universe.push(vec![
        vec![
            vec![false; pocket_hyper_universe[0][0][0].len()];
            pocket_hyper_universe[0][0].len()
        ];
        pocket_hyper_universe[0].len()
    ]);

    for pocket_universe in &mut pocket_hyper_universe {
        pocket_universe.insert(
            0,
            vec![vec![false; pocket_universe[0][0].len()]; pocket_universe[0].len()],
        );
        pocket_universe.push(vec![
            vec![false; pocket_universe[0][0].len()];
            pocket_universe[0].len()
        ]);

        for plane in pocket_universe {
            // insert and append new rows in each plane
            plane.insert(0, vec![false; plane[0].len()]);
            plane.push(vec![false; plane[0].len()]);
            for row in plane {
                // insert and append new columns in each row
                row.insert(0, false);
                row.push(false);
            }
        }
    }

    let hyper_universe_len = pocket_hyper_universe.len();
    let mut new_hyper_universe: Vec<Vec<Vec<Vec<bool>>>> = Vec::new();
    for w in 0..hyper_universe_len {
        let pocket_universe = &pocket_hyper_universe[w];
        let universe_len = pocket_universe.len();

        let mut new_universe: Vec<Vec<Vec<bool>>> = Vec::new();
        for z in 0..universe_len {
            let plane = &pocket_universe[z];
            let plane_len = plane.len();
            let mut new_plane: Vec<Vec<bool>> = Vec::new();

            for y in 0..plane_len {
                let row = &plane[y];
                let row_len = row.len();
                let mut new_row: Vec<bool> = Vec::new();

                for x in 0..row_len {
                    let column = row[x];
                    let mut active_count = 0;

                    for dw in -1..=1 {
                        for dz in -1..=1 {
                            for dy in -1..=1 {
                                for dx in -1..=1 {
                                    if !(dw == 0 && dz == 0 && dy == 0 && dx == 0) {
                                        let wp = w as i128 + dw;
                                        let zp = z as i128 + dz;
                                        let yp = y as i128 + dy;
                                        let xp = x as i128 + dx;
                                        if wp > 0
                                            && wp < (hyper_universe_len as i128)
                                            && zp > 0
                                            && zp < (universe_len as i128)
                                            && yp > 0
                                            && yp < (plane_len as i128)
                                            && xp > 0
                                            && xp < (row_len as i128)
                                        {
                                            if pocket_hyper_universe[wp as usize][zp as usize]
                                                [yp as usize]
                                                [xp as usize]
                                            {
                                                active_count += 1;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    let new_value = if column {
                        if active_count == 2 || active_count == 3 {
                            true
                        } else {
                            false
                        }
                    } else {
                        if active_count == 3 {
                            true
                        } else {
                            false
                        }
                    };

                    new_row.push(new_value);
                }

                new_plane.push(new_row);
            }

            new_universe.push(new_plane);
        }

        new_hyper_universe.push(new_universe);
    }

    new_hyper_universe
}

fn puzzle_1() -> usize {
    let input = load_input();
    // Let's make array order be planes (z), rows (y), columns(x). I think that will be simpler for iteration.

    // First, convert the input into a 3-dimensional vector of booleans
    let mut pocket_universe: Vec<Vec<Vec<bool>>> = vec![input
        .split('\n')
        .map(|row| row.chars().map(|cube| cube == '#').collect())
        .collect()];

    for _ in 0..6 {
        pocket_universe = execute_cycle(&pocket_universe);
    }

    let active_cube_count = pocket_universe.iter().fold(0, |sum, plane| {
        sum + plane.iter().fold(0, |plane_sum, row| {
            plane_sum + row.iter().filter(|c| **c).count()
        })
    });

    active_cube_count
}

fn puzzle_2() -> usize {
    let input = load_input();

    let mut pocket_hyper_universe: Vec<Vec<Vec<Vec<bool>>>> = vec![vec![input
        .split('\n')
        .map(|row| row.chars().map(|cube| cube == '#').collect())
        .collect()]];

    for _ in 0..6 {
        pocket_hyper_universe = execute_hyper_cycle(&pocket_hyper_universe);
    }

    let active_cube_count = pocket_hyper_universe.iter().fold(0, |sum, universe| {
        sum + universe.iter().fold(0, |universe_sum, plane| {
            universe_sum + plane.iter().fold(0, |plane_sum, row| {
                plane_sum + row.iter().filter(|c| **c).count()
            })
        })
    });

    active_cube_count
}

fn main() {
    let start = std::time::Instant::now();
    let result = puzzle_1();
    let elapsed = start.elapsed().as_millis();
    println!("Puzzle 1 result: {}, took {}ms", result, elapsed);

    let start = std::time::Instant::now();
    let result = puzzle_2();
    let elapsed = start.elapsed().as_millis();
    println!("Puzzle 2 result: {}, took {}ms", result, elapsed);
}
