type Cache = std::collections::HashMap<(usize, usize), usize>;

// start si at least from ci
fn odds(config: &Vec<char>, segments: &Vec<usize>, ci: usize, si: usize, margin: usize, cache: &mut Cache) -> usize {
    let key = (ci, si);
    if cache.contains_key(&key) {
        return cache[&key];
    }

    let mut o = 0;
    for m in 0..=margin {
        let margin_is_empty = config[ci..ci+m].iter().all(|&c| c != '#');

        if !margin_is_empty {
            continue;
        }

        let fits = (0..segments[si]).all(|i| config[ci + m + i] != '.');
        if !fits {
            continue;
        }

        if si + 1 == segments.len() {
            let ends_in_empty = config[ci + m + segments[si]..].iter().all(|&c| c != '#');
            if ends_in_empty {
                o += 1;
            }
            continue;
        }

        let spaced_after = config[ci + m + segments[si]] != '#';
        if spaced_after {
            o += odds(config, segments, ci + m + segments[si] + 1, si + 1, margin - m, cache);
        }
    }

    cache.insert(key, o);
    return cache[&key];
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;

    for line in input.lines() {
        let mut parts = line.split(" ");
        let config = parts.next().unwrap().chars().collect::<Vec<_>>();
        let segments = parts.next().unwrap().split(",").map(|p| p.parse::<usize>().unwrap()).collect::<Vec<_>>();

        let minlen = segments.iter().sum::<usize>() + segments.len() - 1;
        let margin = config.len() - minlen;

        let mut cache = Cache::new();
        p1 += odds(&config, &segments, 0, 0, margin, &mut cache);

        let mut cx5 = config.clone();
        let mut sx5 = segments.clone();
        for _ in 0..4 {
            cx5.append(&mut vec!['?']);
            cx5.append(&mut config.clone());
            sx5.append(&mut segments.clone());
        }

        let minlen = sx5.iter().sum::<usize>() + sx5.len() - 1;
        let margin = cx5.len() - minlen;

        let mut cache = Cache::new();
        p2 += odds(&cx5, &sx5, 0, 0, margin, &mut cache);
    }

    (p1, p2)
}

// ???##?###????? 1,2,3,4
// ???##?###?????????##?###?????????##?###?????????##?###?????????##?###?????
// 1,2,3,4,1,2,3,4,1,2,3,4,1,2,3,4,1,2,3,4


