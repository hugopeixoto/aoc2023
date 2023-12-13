peg::parser! {
    grammar node() for str {
        rule label() -> &'input str
            = n:$(['A'..='Z']+) { n }

        pub rule node() -> (&'input str, &'input str, &'input str)
            = id:(label()) " = ("  l:(label()) ", " r:(label()) ")" { (id, l, r) }
    }
}


#[derive(Debug, PartialEq, Eq, Clone)]
enum Type {
    Start,
    End,
    Middle,
}

fn find_loop(mut id: usize, seq: &Vec<char>, tree: &Vec<(usize, usize, Type)>) -> usize {
    let mut steps = 0;
    loop {
        let next = if seq[steps % seq.len()] == 'R' { tree[id].1 } else { tree[id].0 };
        steps += 1;
        id = next;

        if tree[id].2 == Type::End {
            return steps;
        }
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut lines = input.lines();

    let seq = lines.next().unwrap().chars().collect::<Vec<_>>();
    lines.next().unwrap();

    let mut ids = std::collections::HashMap::new();
    let mut tree = Vec::new();
    for line in lines {
        let (id, left, right) = node::node(line).unwrap();
        let nid = { let l = ids.len(); *ids.entry(id).or_insert(l) };
        let left = { let l = ids.len(); *ids.entry(left).or_insert(l) };
        let right = { let l = ids.len(); *ids.entry(right).or_insert(l) };

        let maxid = nid.max(left).max(right);
        if maxid >= tree.len() {
            tree.resize(maxid + 1, (0, 0, Type::Middle));
        }

        tree[nid] = (
            left,
            right,
            if id.ends_with('A') { Type::Start } else if id.ends_with('Z') { Type::End } else { Type::Middle },
        );
    }

    let mut start = ids["AAA"];
    let mut steps = 0;
    loop {
        start = if seq[steps % seq.len()] == 'R' { tree[start].1 } else { tree[start].0 };
        steps += 1;
        if tree[start].2 == Type::End {
            break;
        }
    }
    let p1 = steps;

    let mut x = 1;
    for id in 0..ids.len() {
        if tree[id].2 == Type::Start {
            let zloop = find_loop(id, &seq, &tree);

            // lucky for me zloop.0.len() == 1
            x = num::integer::lcm(x, zloop);
        }
    }

    (p1, x)
}
