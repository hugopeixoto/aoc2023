
#[derive(Debug, Default)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl Draw {
    pub fn power(&self) -> usize {
        self.red * self.green * self.blue
    }

    pub fn is_possible_with(&self, draw: &Self) -> bool {
        self.red <= draw.red &&
            self.green <= draw.green &&
            self.blue <= draw.blue
    }

    pub fn parse(line: &str) -> Self {
        let mut draw = Draw::default();
        for balls in line.split(",") {
            if balls.ends_with("red") { draw.red = balls.trim().split(" ").next().unwrap().parse::<usize>().unwrap(); }
            if balls.ends_with("green") { draw.green = balls.trim().split(" ").next().unwrap().parse::<usize>().unwrap(); }
            if balls.ends_with("blue") { draw.blue = balls.trim().split(" ").next().unwrap().parse::<usize>().unwrap(); }
        }

        draw
    }
}

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl Game {
    pub fn parse(line: &str) -> Self {
        let mut parts = line.split(":");
        let id = parts.next().unwrap().split(" ").last().unwrap().parse::<usize>().unwrap();
        let draws = parts.next().unwrap().split(";").map(Draw::parse).collect();

        Game { id, draws }
    }

    pub fn is_possible_with(&self, draw: &Draw) -> bool {
        self.draws.iter().all(|d| d.is_possible_with(draw))
    }

    pub fn minimum_draw(&self) -> Draw {
        self.draws.iter().fold(Draw::default(), |mut acc, elt| {
            acc.red = acc.red.max(elt.red);
            acc.green = acc.green.max(elt.green);
            acc.blue = acc.blue.max(elt.blue);
            acc
        })
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let games = input.lines()
        .map(|line| Game::parse(line))
        .collect::<Vec<_>>();

    let draw = Draw { red: 12, green: 13, blue: 14 };
    let p1 = games.iter().filter(|g| g.is_possible_with(&draw)).map(|g| g.id).sum();

    let p2 = games.iter().map(|g| g.minimum_draw().power()).sum();

    (p1, p2)
}
