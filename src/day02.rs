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

pub fn solve_easy(input: &String) -> (usize, usize) {
    let games = input.lines()
        .map(|line| Game::parse(line))
        .collect::<Vec<_>>();

    let draw = Draw { red: 12, green: 13, blue: 14 };
    let p1 = games.iter().filter(|g| g.is_possible_with(&draw)).map(|g| g.id).sum();

    let p2 = games.iter().map(|g| g.minimum_draw().power()).sum();

    (p1, p2)
}

pub fn solve(input: &String) -> (usize, usize) {
    //solve_easy(input)
    solve_hard(input)
}

/* a different approach, using a state machine and only looking at each character once */

#[derive(Debug)]
enum State {
    GameID,
    DrawNumber,
    DrawColorSkip,
}

pub fn solve_hard(input: &String) -> (usize, usize) {
    let mut game_id = 0;
    let mut balls = 0;
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    let mut power_sum = 0;
    let mut impossible_id_sum = 0;
    let mut state = State::GameID;

    for c in input.chars() {
        match (&state, c) {
            (State::GameID, '0') => { game_id = game_id * 10 + 0; state = State::GameID; },
            (State::GameID, '1') => { game_id = game_id * 10 + 1; state = State::GameID; },
            (State::GameID, '2') => { game_id = game_id * 10 + 2; state = State::GameID; },
            (State::GameID, '3') => { game_id = game_id * 10 + 3; state = State::GameID; },
            (State::GameID, '4') => { game_id = game_id * 10 + 4; state = State::GameID; },
            (State::GameID, '5') => { game_id = game_id * 10 + 5; state = State::GameID; },
            (State::GameID, '6') => { game_id = game_id * 10 + 6; state = State::GameID; },
            (State::GameID, '7') => { game_id = game_id * 10 + 7; state = State::GameID; },
            (State::GameID, '8') => { game_id = game_id * 10 + 8; state = State::GameID; },
            (State::GameID, '9') => { game_id = game_id * 10 + 9; state = State::GameID; },
            (State::GameID, ':') => { state = State::DrawNumber; },
            (State::DrawNumber, '0') => { balls = balls * 10 + 0; },
            (State::DrawNumber, '1') => { balls = balls * 10 + 1; },
            (State::DrawNumber, '2') => { balls = balls * 10 + 2; },
            (State::DrawNumber, '3') => { balls = balls * 10 + 3; },
            (State::DrawNumber, '4') => { balls = balls * 10 + 4; },
            (State::DrawNumber, '5') => { balls = balls * 10 + 5; },
            (State::DrawNumber, '6') => { balls = balls * 10 + 6; },
            (State::DrawNumber, '7') => { balls = balls * 10 + 7; },
            (State::DrawNumber, '8') => { balls = balls * 10 + 8; },
            (State::DrawNumber, '9') => { balls = balls * 10 + 9; },
            (State::DrawNumber, 'r') => { red = balls; balls = 0; state = State::DrawColorSkip; },
            (State::DrawNumber, 'g') => { green = balls; balls = 0; state = State::DrawColorSkip; },
            (State::DrawNumber, 'b') => { blue = balls; balls = 0; state = State::DrawColorSkip; },
            (State::DrawColorSkip, ',') => { state = State::DrawNumber; }, // end of balls
            (State::DrawColorSkip, ';') => {
                max_red = max_red.max(red);
                max_green = max_green.max(green);
                max_blue = max_blue.max(blue);

                red = 0; green = 0; blue = 0;
                state = State::DrawNumber;
            }, // end of draw
            (State::DrawColorSkip, '\n') => {
                max_red = max_red.max(red);
                max_green = max_green.max(green);
                max_blue = max_blue.max(blue);

                red = 0; green = 0; blue = 0;

                power_sum += max_red * max_green * max_blue;

                if max_red <= 12 && max_green <= 13 && max_blue <= 14 {
                    impossible_id_sum += game_id;
                }

                max_red = 0; max_green = 0; max_blue = 0;

                game_id = 0;
                state = State::GameID;
            }, // end of game
            _ => { }
        }
    }

    (impossible_id_sum, power_sum)
}
