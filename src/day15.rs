fn hash(word: &str) -> usize {
    let mut h = 0;
    for c in word.chars() {
        h = ((h + c as usize)*17) % 256;
    }

    h
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];

    let mut p1 = 0;
    for instruction in input.trim().split(",") {
        let ph = hash(instruction);

        if instruction.contains("-") {
            let label = instruction.trim_matches('-');
            let h = hash(label);
            boxes[h] = boxes[h].iter().filter(|lens| lens.label != label).cloned().collect();
        } else {
            let mut parts = instruction.split("=");
            let label = parts.next().unwrap().to_string();
            let h = hash(&label);
            let focal_length = parts.next().unwrap().parse::<usize>().unwrap();
            let lens = Lens { label: label.clone(), focal_length };

            let pos = boxes[h].iter_mut().find(|l| l.label == label);
            if pos.is_none() {
                boxes[h].push(lens);
            } else {
                pos.unwrap().focal_length = focal_length;
            }
        }

        p1 += ph;
    }

    let mut p2 = 0;
    for (bi, b) in boxes.iter().enumerate() {
        for (li, lens) in b.iter().enumerate() {
            let power = (bi + 1) * (li + 1) * lens.focal_length;
            p2 += power;
        }
    }

    (p1, p2)
}
