fn parse_vec_usize(line: &str) -> Vec<usize> {
    line.split(" ").filter_map(|p| p.parse::<usize>().ok()).collect()
}

// -x² + tx - d > 0
// -x² + tx - d = 0
// x = (-b ± sqrt(b² - 4ac)) / 2a
// x = (-t ± sqrt(t² - 4*1*d)) / 2*(-1)
// x = (-t ± sqrt(t² - 4d)) / -2
// x = (t ± sqrt(t² - 4d)) / 2

fn next_integer(x: f64) -> usize { return (x + 1.0).floor() as usize }
fn prev_integer(x: f64) -> usize { return (x - 1.0).ceil() as usize }
fn number_of_ways(time: usize, distance: usize) -> usize {
    let sq = ((time*time - 4*distance) as f64).sqrt();

    let lower = next_integer((time as f64 - sq) / 2.0);
    let upper = prev_integer((time as f64 + sq) / 2.0);

    upper - lower + 1
}

pub fn solve(input: &String) -> (usize, usize) {
    let games = input.lines().map(|line| {
        parse_vec_usize(line.split(":").last().unwrap())
    }).collect::<Vec<_>>();

    let p1 = (0..games[0].len()).map(|i| number_of_ways(games[0][i], games[1][i])).product();

    let game = input.lines().map(|line| {
        line.split(":").last().unwrap().replace(" ", "").parse::<usize>().unwrap()
    }).collect::<Vec<_>>();

    let p2 = number_of_ways(game[0], game[1]);

    (p1, p2)
}
