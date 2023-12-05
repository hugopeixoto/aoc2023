use itertools::Itertools;

pub fn parse_vec_usize(line: &str) -> Vec<usize> {
    line.split(" ").filter_map(|p| p.parse::<usize>().ok()).collect()
}

#[derive(Debug)]
struct Mapper {
    source: usize,
    target: usize,
    length: usize,
}

impl Mapper {
    pub fn new(source: usize, target: usize, length: usize) -> Self {
        Mapper { source, target, length }
    }
    pub fn map(&self, value: usize) -> Option<usize> {
        if self.source <= value  && value < self.source + self.length {
            Some((value - self.source) + self.target)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Tx {
    mappers: Vec<Mapper>,
}

impl Tx {
    pub fn new() -> Self { Self { mappers: vec![] } }
    pub fn map(&self, mut seed_range: (usize, usize)) -> Vec<(usize, usize)> {
        let mut results = vec![];
        let mut idx = 0;
        while seed_range.1 > 0 && idx < self.mappers.len() {
            let mapper = &self.mappers[idx];
            if seed_range.0 < mapper.source {
                let length = seed_range.1.min(mapper.source - seed_range.0);
                results.push((seed_range.0, length));
                seed_range = (mapper.source, seed_range.1 - length);
            } else if mapper.source + mapper.length <= seed_range.0 {
                idx += 1;
            } else {
                let start = mapper.map(seed_range.0).unwrap();
                let length = (seed_range.0 + seed_range.1).min(mapper.source + mapper.length) - seed_range.0;

                results.push((start, length));
                seed_range = (seed_range.0 + length, seed_range.1 - length);
            }
        }

        if seed_range.1 > 0 {
            results.push(seed_range);
        }

        results
    }

    pub fn push(&mut self, tx: Mapper) {
        self.mappers.push(tx);
    }
    pub fn finalize(&mut self) {
        self.mappers.sort_by_key(|m| m.source);
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut lines = input.lines();

    let seeds = parse_vec_usize(lines.next().unwrap().split(":").last().unwrap());
    let mut maps = vec![];
    for line in lines {
        if line == "" {
        } else if line.ends_with("map:") {
            maps.push(Tx::new());
        } else {
            let x = parse_vec_usize(line);
            maps.last_mut().unwrap().push(Mapper::new(x[1], x[0], x[2]));
        }
    }

    for tx in maps.iter_mut() { tx.finalize(); }

    let p1 = seeds.iter().map(|seed| {
        maps.iter().fold(seed.clone(), |s, tx| {
            tx.map((s, 1))[0].0
        })
    }).min().unwrap_or(0);

    let p2 = seeds.iter().tuples().map(|(seed, count)| {
        maps.iter().fold(vec![(*seed, *count)], |seed_ranges, tx| {
            seed_ranges.into_iter().flat_map(|seed_range| {
                tx.map(seed_range)
            }).collect()
        }).iter().min().unwrap().0
    }).min().unwrap_or(0);

    (p1, p2)
}
