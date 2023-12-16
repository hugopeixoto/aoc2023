use itertools::Itertools;

pub fn parse_vec_usize(line: &str) -> Vec<i64> {
    line.split(" ").filter_map(|p| p.parse::<i64>().ok()).collect()
}

pub fn predict(sequence: &Vec<i64>) -> (i64, i64) {
    if sequence.iter().all(|&x| x == 0) {
        (0, 0)
    } else {
        let down = sequence.iter().tuple_windows().map(|(a,b)| b - a).collect();
        let (p, n) = predict(&down);
        (sequence[0] - p, sequence.last().unwrap() + n)
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;
    for line in input.lines() {
        let sequence = parse_vec_usize(line);

        let (prev, next) = predict(&sequence);

        p1 += next;
        p2 += prev;
    }

    (p1 as usize, p2 as usize)
}
