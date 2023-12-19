
#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn apply(&self, beam: &Beam) -> Beam {
        match self {
            Self::North => Beam { coords: (beam.x(), beam.y() - 1), direction: self.clone() },
            Self::South => Beam { coords: (beam.x(), beam.y() + 1), direction: self.clone() },
            Self::West => Beam { coords: (beam.x() - 1, beam.y()), direction: self.clone() },
            Self::East => Beam { coords: (beam.x() + 1, beam.y()), direction: self.clone() },
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Beam {
    coords: (i64, i64),
    direction: Direction,
}

impl Beam {
    pub fn x(&self) -> i64 {
        self.coords.0
    }
    pub fn y(&self) -> i64 {
        self.coords.1
    }
}

fn in_matrix(beam: &Beam, matrix: &Vec<Vec<char>>) -> bool {
    if 0 <= beam.coords.0 && beam.coords.0 < matrix[0].len() as i64 {
        if 0 <= beam.coords.1 && beam.coords.1 < matrix.len() as i64 {
            return true;
        }
    }

    return false;
}

pub fn energized(b: Beam, matrix: &Vec<Vec<char>>) -> usize {
    let mut seen = std::collections::HashSet::new();
    let mut beams = std::collections::VecDeque::new();

    beams.push_back(b);
    while !beams.is_empty() {
        let beam = beams.pop_front().unwrap();

        if !in_matrix(&beam, &matrix) {
            continue;
        }
        if seen.contains(&beam) {
            continue;
        }
        seen.insert(beam.clone());

        match (&beam.direction, matrix[beam.y() as usize][beam.x() as usize]) {
            (Direction::East|Direction::West, '-') => { beams.push_back(beam.direction.apply(&beam)); },
            (Direction::North|Direction::South, '|') => { beams.push_back(beam.direction.apply(&beam)); },
            (_, '.') => { beams.push_back(beam.direction.apply(&beam)); },

            (Direction::North|Direction::South, '-') => {
                beams.push_back(Direction::West.apply(&beam));
                beams.push_back(Direction::East.apply(&beam));
            },
            (Direction::East|Direction::West, '|') => {
                beams.push_back(Direction::North.apply(&beam));
                beams.push_back(Direction::South.apply(&beam));
            },
            (Direction::East, '/') => { beams.push_back(Direction::North.apply(&beam)); },
            (Direction::West, '/') => { beams.push_back(Direction::South.apply(&beam)); },
            (Direction::North, '/') => { beams.push_back(Direction::East.apply(&beam)); },
            (Direction::South, '/') => { beams.push_back(Direction::West.apply(&beam)); },
            (Direction::East, '\\') => { beams.push_back(Direction::South.apply(&beam)); },
            (Direction::West, '\\') => { beams.push_back(Direction::North.apply(&beam)); },
            (Direction::North, '\\') => { beams.push_back(Direction::West.apply(&beam)); },
            (Direction::South, '\\') => { beams.push_back(Direction::East.apply(&beam)); },

            _ => { panic!("poop"); }
        }
    }

    seen.iter().map(|b| b.coords).collect::<std::collections::HashSet::<(i64,i64)>>().len()
}

pub fn solve(input: &String) -> (usize, usize) {
    let matrix = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let w = matrix[0].len() as i64;
    let h = matrix.len() as i64;

    let p1 = energized(Beam { coords: (0,0), direction: Direction::East }, &matrix);

    let mut p2 = 0;
    for y in 0..matrix.len() {
        let e1 = energized(Beam { coords: (0, y as i64), direction: Direction::East }, &matrix);
        let e2 = energized(Beam { coords: (w - 1, y as i64), direction: Direction::West }, &matrix);

        p2 = p2.max(e1).max(e2);
    }
    for x in 0..matrix[0].len() {
        let e1 = energized(Beam { coords: (x as i64, 0), direction: Direction::South }, &matrix);
        let e2 = energized(Beam { coords: (x as i64, h - 1), direction: Direction::North }, &matrix);

        p2 = p2.max(e1).max(e2);
    }

    (p1, p2)
}
