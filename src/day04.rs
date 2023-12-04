pub fn parse_vec_usize(line: &str) -> std::collections::HashSet<usize> {
    line.split(" ").filter_map(|p| p.parse::<usize>().ok()).collect()
}

pub fn parse_scratch_card(line: &str) -> (usize, usize) {
    let mut parts = line.split(":");
    let id = parts.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();

    let mut parts = parts.next().unwrap().split("|");
    let winners = parse_vec_usize(parts.next().unwrap());
    let numbers = parse_vec_usize(parts.next().unwrap());
    let matches = winners.intersection(&numbers).count();

    (id, matches)
}

pub fn points(n: usize) -> usize {
    if n == 0 {
        0
    } else {
        1 << (n - 1)
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    let mut queue = vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for (index, line) in input.lines().enumerate() {
        let (_, matches) = parse_scratch_card(&line);

        p1 += points(matches);

        let copies = 1 + queue[index % 11];
        for i in 1..=matches {
             queue[(index + i)%11] += copies;
        }
        p2 += copies;
        queue[index % 11] = 0;
    }

    (p1, p2)
}
