use itertools::Itertools;

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
}


fn parse1(line: &str) -> (Direction, usize) {
    let mut parts = line.split(" ");

    let dir = match parts.next() {
        Some("U") => Direction::North,
        Some("D") => Direction::South,
        Some("R") => Direction::East,
        Some("L") => Direction::West,
        _ => { panic!(); }
    };

    let meters = parts.next().unwrap().parse::<usize>().unwrap();

    (dir, meters)
}

fn parse2(line: &str) -> (Direction, usize) {
    let color = line.split(" ").last().unwrap();

    let dir = match &color[7..8] {
        "3" => Direction::North,
        "1" => Direction::South,
        "0" => Direction::East,
        "2" => Direction::West,
        _ => { panic!(); }
    };

    let meters = usize::from_str_radix(&color[2..7], 16).unwrap();

    (dir, meters)
}

fn area(points: &Vec<(i64, i64)>) -> i64 {
    let mut area: i64 = 0;
    let mut edges = 0;
    for (p,q) in points.iter().tuple_windows() {
        let da = (p.0*q.1 - p.1*q.0) * 4;
        area += da;

        edges += ((p.0.abs_diff(q.0) + p.1.abs_diff(q.1)) as i64) * 4;

        //area += 1;
    }

    edges += 4*2;

    if area < 0 {
        area -= edges;
    } else {
        area += edges;
    }

    area / 8
}

pub fn solve(input: &String) -> (usize, usize) {
    let p1;
    let p2;
    {
        let mut p = (0, 0);
        let mut points = vec![p];
        for inst in input.lines().map(parse1) {
            let q = inst.0.apply(&p, inst.1 as i64);
            points.push(q);
            p = q;
        }

        points.push(points[0]);

        p1 = area(&points) as usize;
    }

    {
        let mut p = (0, 0);
        let mut points = vec![p];
        for inst in input.lines().map(parse2) {
            let q = inst.0.apply(&p, inst.1 as i64);
            points.push(q);
            p = q;
        }

        points.push(points[0]);

        p2 = area(&points) as usize;
    }

    (p1, p2)
}
