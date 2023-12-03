pub fn solve(input: &String) -> (usize, usize) {
    let symbols = input.chars().collect::<Vec<_>>();
    let width = symbols.iter().position(|c| *c == '\n').unwrap();
    let height = symbols.len() / (width + 1);

    let is_symbol = |c: char| !(c.is_digit(10) || c == '.' || c == '\n');
    let char_at = |x: i32, y: i32, symbols: &Vec<char>| {
        symbols[(y as usize)*(width+1)+x as usize]
    };
    let in_bounds = |x: i32, y: i32| {
        0 <= x && x < (width as i32) &&
        0 <= y && y < (height as i32)
    };

    let mut numbers = std::collections::HashSet::new();
    let mut gear_ratios = 0;
    for idx in 0.. symbols.len() {
        if is_symbol(symbols[idx]) {
            let y = (idx / (width + 1)) as i32;
            let x = (idx % (width + 1)) as i32;

            let mut gear_numbers = std::collections::HashSet::new();
            for dx in [-1, 0, 1] {
                for dy in [-1, 0, 1] {
                    let mut nx = x + dx;
                    let ny = y + dy;
                    if in_bounds(nx, ny) && char_at(nx, ny, &symbols).is_digit(10) {
                        while in_bounds(nx - 1, ny) && char_at(nx-1, ny, &symbols).is_digit(10) {
                            nx -= 1;
                        }

                        let mut n = (nx, ny, 0);
                        while char_at(nx, ny, &symbols).is_digit(10) {
                            n.2 = n.2 * 10 + char_at(nx, ny, &symbols).to_digit(10).unwrap();
                            nx += 1;
                        }

                        numbers.insert(n);

                        if symbols[idx] == '*' {
                            gear_numbers.insert(n);
                        }
                    }
                }
            }

            if gear_numbers.len() == 2 {
                gear_ratios += gear_numbers.iter().fold(1, |a,b| a * b.2);
            }
        }
    }

    let p1 = numbers.iter().map(|v| v.2 as usize).sum();

    (p1, gear_ratios as usize)
}
