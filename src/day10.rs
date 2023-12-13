
fn neighbors((x,y): (usize, usize), matrix: &Vec<Vec<(char, bool)>>) -> Option<((usize, usize), (usize, usize))> {
    match matrix[y][x].0 {
        '-' => { Some(((x-1, y), (x+1, y))) },
        '|' => { Some(((x, y-1), (x, y+1))) },
        'L' => { Some(((x, y-1), (x+1, y))) },
        'J' => { Some(((x, y-1), (x-1, y))) },
        '7' => { Some(((x-1, y), (x, y+1))) },
        'F' => { Some(((x+1, y), (x, y+1))) },
        '.' => { None },
        _ => { panic!("huh? {:?}", matrix[y][x]); }
    }
}

fn real_s((x,y): (usize, usize), matrix: &Vec<Vec<(char, bool)>>) -> char {
    let up = y > 0 && (matrix[y-1][x].0 == '|' || matrix[y-1][x].0 == 'F' || matrix[y-1][x].0 == '7');
    let down = y + 1 < matrix.len() && (matrix[y+1][x].0 == '|' || matrix[y+1][x].0 == 'L' || matrix[y+1][x].0 == 'J');
    let left = x > 0 && (matrix[y][x-1].0 == '-' || matrix[y][x-1].0 == 'L' || matrix[y][x-1].0 == 'F');
    let right = x + 1 < matrix[0].len() && (matrix[y][x+1].0 == '-' || matrix[y][x+1].0 == 'J' || matrix[y][x+1].0 == '7');

    match (up, down, left, right) {
        (true, true, false, false) => '|',
        (false, false, true, true) => '-',
        (true, false, true, false) => 'J',
        (true, false, false, true) => 'L',
        (false, true, true, false) => '7',
        (false, true, false, true) => 'F',
        _ => { panic!(); }
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut matrix = input.lines().map(|line| line.chars().map(|c| (c, false)).collect::<Vec<_>>()).collect::<Vec<_>>();
    let width = matrix[0].len();
    let height = matrix.len();

    let mut start = (0, 0);
    for x in 0..width {
        for y in 0..height {
            if matrix[y][x].0 == 'S' {
                start = (x, y);
            }
        }
    }

    matrix[start.1][start.0].0 = real_s(start, &matrix);

    let mut prev = start;
    let mut next = neighbors(prev, &matrix).unwrap().0;
    let mut steps = 1;
    matrix[prev.1][prev.0].1 = true;
    while next != start {
        matrix[next.1][next.0].1 = true;
        let (n1, n2) = neighbors(next, &matrix).unwrap();
        if prev == n1 {
            prev = next;
            next = n2;
        } else {
            prev = next;
            next = n1;
        }
        steps += 1;
    }

    let mut inside = 0;
    for y in 0..height {
        let mut state = S::Outside;
        let mut from = F::Nowhere;
        for x in 0..width {
            if matrix[y][x].1 {
                match (&state, &from, matrix[y][x].0) {
                    (S::Outside | S::Inside, _, '|') => { state = state.flip(); }
                    (_, F::Nowhere, 'F') => { from = F::Down; }
                    (_, F::Nowhere, 'L') => { from = F::Up; }
                    (_, F::Up|F::Down, '-') => {},
                    (_, F::Up, 'J') => { from = F::Nowhere; },
                    (_, F::Down, '7') => { from = F::Nowhere; },
                    (_, F::Up, '7') => { from = F::Nowhere; state = state.flip(); },
                    (_, F::Down, 'J') => { from = F::Nowhere; state = state.flip(); },
                    _ => { panic!("{:?} {:?} {:?}", state, from, matrix[y][x]); }
                }
            } else if state == S::Inside {
                inside += 1;
            }

        }
    }

    (steps/2, inside)
}

#[derive(Debug, PartialEq, Eq)]
enum F {
    Nowhere,
    Up,
    Down,
}

#[derive(Debug, PartialEq, Eq)]
enum S {
    Outside,
    Inside,
}
impl S {
    pub fn flip(&self) -> Self {
        match self {
            Self::Outside => Self::Inside,
            Self::Inside => Self::Outside,
        }
    }
}
