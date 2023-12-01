#[derive(Default)]
struct State {
    pub first: Option<u32>,
    pub last: Option<u32>,
    pub state: u32,
}

impl State {
    pub fn next(mut self, ch: char) -> Self {
        match (self.state, ch) {
            (_, '1') => { self.read(1); },
            (_, '2') => { self.read(2); },
            (_, '3') => { self.read(3); },
            (_, '4') => { self.read(4); },
            (_, '5') => { self.read(5); },
            (_, '6') => { self.read(6); },
            (_, '7') => { self.read(7); },
            (_, '8') => { self.read(8); },
            (_, '9') => { self.read(9); },
            (10|41, 'n') => { self.state = 11; },
            (11, 'e') => { self.read(1); self.state = 80; },
            (20, 'w') => { self.state = 21; },
            (21, 'o') => { self.read(2); self.state = 10; },
            (20, 'h') => { self.state = 31; },
            (31, 'r') => { self.state = 32; },
            (32, 'e') => { self.state = 33; },
            (33, 'e') => { self.read(3); self.state = 80; },
            (40, 'o') => { self.state = 41; },
            (41, 'u') => { self.state = 42; },
            (42, 'r') => { self.read(4); self.state = 0; },
            (40, 'i') => { self.state = 51; },
            (51, 'v') => { self.state = 52; },
            (52, 'e') => { self.read(5); self.state = 80; },
            (60, 'i') => { self.state = 61; },
            (61, 'x') => { self.read(6); self.state = 0; },
            (60, 'e') => { self.state = 71; },
            (71, 'v') => { self.state = 72; },
            (72, 'e') => { self.state = 73; },
            (73, 'n') => { self.read(7); self.state = 90; },
            (80|33|71|73, 'i') => { self.state = 81; },
            (81, 'g') => { self.state = 82; },
            (82, 'h') => { self.state = 83; },
            (83, 't') => { self.read(8); self.state = 20; },
            (90|11|91, 'i') => { self.state = 91; },
            (91, 'n') => { self.state = 92; },
            (92, 'e') => { self.read(9); self.state = 80; },
            (_, 'o') => { self.state = 10; },
            (_, 't') => { self.state = 20; },
            (_, 'f') => { self.state = 40; },
            (_, 's') => { self.state = 60; },
            (_, 'e') => { self.state = 80; },
            (_, 'n') => { self.state = 90; },
            (_, _) => { self.state = 0; },
        }

        self
    }

    pub fn read(&mut self, n: u32) {
        self.first = self.first.or(Some(n));
        self.last = Some(n);
    }
}

pub fn solve(input: &String) -> (usize, usize) {
    let p1: u32 = input.lines().map(|line| {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first = digits.next();
        let last = digits.last().or(first);

        first.unwrap() * 10 + last.unwrap()
    }).sum();

    let p2: u32 = input.lines().map(|line| {
        let state = line.chars().fold(State::default(), |state, ch| state.next(ch));

        state.first.unwrap() * 10 + state.last.unwrap()
    }).sum();

    (p1 as usize, p2 as usize)
}
