
#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn apply(&self, xy: &(i64, i64), delta: i64) -> (i64, i64) {
        match self {
            Self::North => (xy.0, xy.1 - delta),
            Self::South => (xy.0, xy.1 + delta),
            Self::West => (xy.0 - delta, xy.1),
            Self::East => (xy.0 + delta, xy.1),
        }
    }

    fn rot(&self) -> Self {
        match self {
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
            Self::West => Self::North,
        }
    }
    fn nrot(&self) -> Self {
        self.rot().rot().rot()
    }
}

#[derive(Debug, PartialEq, Eq, Ord, Clone)]
struct State {
    heat_loss: usize,
    position: (i64, i64),
    direction: Direction,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.heat_loss.partial_cmp(&self.heat_loss)
    }
}

impl State {
    fn adv(&self, matrix: &Vec<Vec<usize>>) -> Option<Self> {
        let pos = self.direction.apply(&self.position, 1);

        if 0 <= pos.0 && pos.0 < matrix[0].len() as i64 && 0 <= pos.1 && pos.1 < matrix.len() as i64 {
            Some(Self {
                heat_loss: self.heat_loss + matrix[pos.1 as usize][pos.0 as usize],
                position: pos,
                direction: self.direction.clone(),
            })
        } else {
            None
        }
    }

    fn rot(&self, matrix: &Vec<Vec<usize>>) -> Option<Self> {
        Self {
            heat_loss: self.heat_loss,
            position: self.position,
            direction: self.direction.rot(),
        }.adv(matrix)
    }

    fn nrot(&self, matrix: &Vec<Vec<usize>>) -> Option<Self> {
        Self {
            heat_loss: self.heat_loss,
            position: self.position,
            direction: self.direction.nrot(),
        }.adv(matrix)
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let matrix = input.lines().map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<_>>()).collect::<Vec<_>>();
    let w = matrix[0].len();
    let h = matrix.len();

    let mut p1 = 0;
    {
        let mut queue = std::collections::BinaryHeap::new();
        let mut seen = std::collections::HashSet::new();

        queue.push(State { heat_loss: 0, position: (0, 0), direction: Direction::South });
        queue.push(State { heat_loss: 0, position: (0, 0), direction: Direction::East });

        while !queue.is_empty() {
            let s = queue.pop().unwrap();

            let key = (s.position, s.direction.clone());
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            if s.position == (w as i64 -1, h as i64-1) {
                p1 = s.heat_loss;
                break;
            }

            if let Some(t) = s.rot(&matrix) { queue.push(t.clone());
                if let Some(t2) = t.adv(&matrix) { queue.push(t2.clone());
                    if let Some(t3) = t2.adv(&matrix) { queue.push(t3); }}}
            if let Some(t) = s.nrot(&matrix) { queue.push(t.clone());
                if let Some(t2) = t.adv(&matrix) { queue.push(t2.clone());
                    if let Some(t3) = t2.adv(&matrix) { queue.push(t3); }}}
        }
    }

    let mut p2 = 0;
    {
        let mut queue = std::collections::BinaryHeap::new();
        let mut seen = std::collections::HashSet::new();

        queue.push(State { heat_loss: 0, position: (0, 0), direction: Direction::South });
        queue.push(State { heat_loss: 0, position: (0, 0), direction: Direction::East });

        while !queue.is_empty() {
            let s = queue.pop().unwrap();

            let key = (s.position, s.direction.clone());
            if seen.contains(&key) {
                continue;
            }
            seen.insert(key);

            if s.position == (w as i64 -1, h as i64-1) {
                p2 = s.heat_loss;
                break;
            }

            let mut t = s.rot(&matrix)
                .and_then(|x| x.adv(&matrix))
                .and_then(|x| x.adv(&matrix))
                .and_then(|x| x.adv(&matrix));
            for _ in 0..=10-4 {
                if t.is_some() {
                    queue.push(t.clone().unwrap());
                    t = t.unwrap().adv(&matrix);
                }
            }

            let mut t = s.nrot(&matrix)
                .and_then(|x| x.adv(&matrix))
                .and_then(|x| x.adv(&matrix))
                .and_then(|x| x.adv(&matrix));
            for _ in 0..=10-4 {
                if t.is_some() {
                    queue.push(t.clone().unwrap());
                    t = t.unwrap().adv(&matrix);
                }
            }
        }
    }

    (p1, p2)
}
