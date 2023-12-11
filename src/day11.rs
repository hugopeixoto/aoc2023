pub fn solve(input: &String) -> (usize, usize) {
    let matrix = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    let w = matrix[0].len();
    let h = matrix.len();

    let mut expansions_w = Vec::with_capacity(w);
    let mut expansions_h = Vec::with_capacity(h);

    let mut empties: usize = 0;
    for x in 0..w {
        let empty = matrix.iter().all(|row| row[x] == '.');
        if empty {
            empties += 1;
        }
        expansions_w.push(empties);
    }
    let mut empties: usize = 0;
    for y in 0..h {
        let empty = matrix[y].iter().all(|&v| v == '.');
        if empty {
            empties += 1;
        }
        expansions_h.push(empties);
    }

    let mut galaxies = Vec::with_capacity(w*h);
    for y in 0..h {
        for x in 0..w {
            if matrix[y][x] == '#' {
                galaxies.push((x, y));
            }
        }
    }

    let mut p1 = 0;
    for (i, g) in galaxies.iter().enumerate() {
        for (j, g2) in galaxies[i+1..].iter().enumerate() {
            let dx = g.0.abs_diff(g2.0);
            let dy = g.1.abs_diff(g2.1);

            let ex = expansions_w[g2.0].abs_diff(expansions_w[g.0]);
            let ey = expansions_h[g2.1].abs_diff(expansions_h[g.1]);

            p1 += dx + dy + ex + ey;
        }
    }

    let mut p2 = 0;
    for (i, g) in galaxies.iter().enumerate() {
        for (j, g2) in galaxies[i+1..].iter().enumerate() {

            let dx = g.0.abs_diff(g2.0);
            let dy = g.1.abs_diff(g2.1);

            let ex = expansions_w[g2.0].abs_diff(expansions_w[g.0]);
            let ey = expansions_h[g2.1].abs_diff(expansions_h[g.1]);

            p2 += dx + dy + (ex + ey)*999_999;
        }
    }

    (p1, p2)
}
