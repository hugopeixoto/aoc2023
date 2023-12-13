fn diff(a: &Vec<bool>, b: &Vec<bool>) -> usize {
    (0..a.len()).map(|i| if a[i] == b[i] { 0 } else { 1 }).sum()
}

pub fn solve(input: &String) -> (usize, usize) {
    let mut p1 = 0;
    let mut p2 = 0;
    for pattern in input.split("\n\n") {
        let matrix = pattern.lines().map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>()).collect::<Vec<_>>();
        let w = matrix[0].len();
        let h = matrix.len();

        let mut matrix_t = vec![];
        matrix_t.resize_with(w, || vec![false; h]);
        for x in 0..w {
            for y in 0..h {
                matrix_t[x][y] = matrix[y][x];
            }
        }

        for y in 0..h-1 {
            let mut diffs = 0;
            for y2 in 0..=y {
                if y+1+y-y2 < matrix.len() {
                    diffs += diff(&matrix[y2], &matrix[y+1+y-y2]);
                }
            }

            if diffs == 0 {
                p1 += (y+1)*100;
            }
            if diffs == 1 {
                p2 += (y+1)*100;
            }
        }

        for x in 0..w-1 {
            let mut diffs = 0;
            for x2 in 0..=x {
                if x+1+x-x2 < matrix_t.len() {
                    diffs += diff(&matrix_t[x2], &matrix_t[x+1+x-x2]);
                }
            }

            if diffs == 0 {
                p1 += (x+1)*1;
            }
            if diffs == 1 {
                p2 += (x+1)*1;
            }
        }
    }

    (p1, p2)
}
