enum Direction {
    North,
    West,
    South,
    East,
}

fn spin(direction: Direction, matrix: &mut Vec<Vec<char>>) {
    let w = matrix[0].len();
    let h = matrix.len();
    match direction {
        Direction::North => {
            for y in 0..h {
                for x in 0..w {
                    if matrix[y][x] == 'O' {
                        let mut ry = y;
                        while ry > 0 && matrix[ry-1][x] == '.' {
                            matrix[ry][x] = '.';
                            matrix[ry-1][x] = 'O';
                            ry -= 1;
                        }
                    }
                }
            }
        },
        Direction::South => {
            for iy in 0..h {
                let y = h - 1 - iy;
                for x in 0..w {
                    if matrix[y][x] == 'O' {
                        let mut ry = y;
                        while ry + 1 < h && matrix[ry+1][x] == '.' {
                            matrix[ry][x] = '.';
                            matrix[ry+1][x] = 'O';
                            ry += 1;
                        }
                    }
                }
            }
        },
        Direction::West => {
            for x in 0..w {
                for y in 0..h {
                    if matrix[y][x] == 'O' {
                        let mut rx = x;
                        while rx > 0 && matrix[y][rx-1] == '.' {
                            matrix[y][rx] = '.';
                            matrix[y][rx-1] = 'O';
                            rx -= 1;
                        }
                    }
                }
            }
        },
        Direction::East => {
            for ix in 0..w {
                let x = w - 1 - ix;
                for y in 0..h {
                    if matrix[y][x] == 'O' {
                        let mut rx = x;
                        while rx +1 < w && matrix[y][rx+1] == '.' {
                            matrix[y][rx] = '.';
                            matrix[y][rx+1] = 'O';
                            rx += 1;
                        }
                    }
                }
            }
        },
    }
}

fn load(matrix: &Vec<Vec<char>>) -> usize {
    let w = matrix[0].len();
    let h = matrix.len();

    let mut r = 0;
    for y in 0..h {
        for x in 0..w {
            if matrix[y][x] == 'O' {
                r += h-y;
            }
        }
    }
    r
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut matrix = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut seen = std::collections::HashMap::new();
    let mut loads = std::collections::HashMap::new();

    println!("{:?}", matrix);

    spin(Direction::North, &mut matrix);

    let p1 = load(&matrix);

    spin(Direction::West, &mut matrix);
    spin(Direction::South, &mut matrix);
    spin(Direction::East, &mut matrix);

    seen.insert(matrix.clone(), 1);
    loads.insert(1, load(&matrix));

    let mut p2 = 0;
    for i in 2.. {
        println!("load after {}: {}", i-1, load(&matrix));
        spin(Direction::North, &mut matrix);
        spin(Direction::West, &mut matrix);
        spin(Direction::South, &mut matrix);
        spin(Direction::East, &mut matrix);
        if seen.contains_key(&matrix) {
            println!("loop after {}: {}", i, seen[&matrix]);

            let loop_size = i - seen[&matrix];
            let idx = (1000_000_000 - seen[&matrix]) % loop_size + seen[&matrix];

            println!("{}: {}", idx, loads[&idx]);
            p2 = loads[&idx];
            break;
        }
        seen.insert(matrix.clone(), i);
        loads.insert(i, load(&matrix));
    }

    (p1, p2)
}
