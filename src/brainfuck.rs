use std::io::Read;
use std::io::Write;

enum Instruction {
    Next,
    Prev,
    Inc,
    Dec,
    Read,
    Write,
    If(usize),
    Fi(usize),
    Debug,
}

struct Tape {
    data: Vec<u16>,
    pointer: usize,
}

impl Tape {
    pub fn new() -> Self {
        let mut tape = Self {
            data: vec![],
            pointer: 0,
        };

        tape.data.resize(32_000, 0);
        tape
    }

    pub fn forward(&mut self, offset: usize) { self.pointer += offset; }
    pub fn backward(&mut self, offset: usize) { self.pointer -= offset; }
    pub fn inc(&mut self) { self.data[self.pointer] = self.data[self.pointer].overflowing_add(1).0; }
    pub fn dec(&mut self) { self.data[self.pointer] = self.data[self.pointer].overflowing_sub(1).0; }
    pub fn set(&mut self, v: u16) { self.data[self.pointer] = v; }
    pub fn get(&mut self) -> u16 { self.data[self.pointer] }

    pub fn debug(&self) {
        print!("tape[ ");
        for i in 0..=(self.pointer + 5) {
            if i == self.pointer {
                print!("{:3}|", self.data[i]);
            } else {
                print!("{:3} ", self.data[i]);
            }
        }
        println!("]");
    }
}

pub struct Program {
    code: Vec<Instruction>,
}

impl Program {
    pub fn run(&self) {
        let mut tape = Tape::new();
        let mut ip = 0;

        let mut buf: [u8; 1] = [0];

        while ip < self.code.len() {
            match self.code[ip] {
                Instruction::Next => { tape.forward(1); ip += 1; }
                Instruction::Prev => { tape.backward(1); ip += 1; }
                Instruction::Inc => { tape.inc(); ip += 1; }
                Instruction::Dec => { tape.dec(); ip += 1; }
                Instruction::Read => {
                    buf[0] = 0;
                    let _ = std::io::stdin().read_exact(&mut buf);
                    tape.set(buf[0] as u16);
                    ip += 1;
                }
                Instruction::Write => { buf[0] = tape.get() as u8; std::io::stdout().write(&buf).unwrap(); ip += 1; }
                Instruction::If(offset) => { if tape.get() == 0 { ip = offset; } else { ip += 1; } }
                Instruction::Fi(offset) => { if tape.get() != 0 { ip = offset; } else { ip += 1; } }
                Instruction::Debug => { tape.debug(); ip += 1; }
            }
        }
    }
}

pub fn compile(program: &str) -> Program {
    let mut code = program.chars().filter_map(|c| match c {
        '>' => Some(Instruction::Next),
        '<' => Some(Instruction::Prev),
        '+' => Some(Instruction::Inc),
        '-' => Some(Instruction::Dec),
        ',' => Some(Instruction::Read),
        '.' => Some(Instruction::Write),
        '[' => Some(Instruction::If(0)),
        ']' => Some(Instruction::Fi(0)),
        '@' => Some(Instruction::Debug),
        _ => None
    }).collect::<Vec<_>>();

    // this could be done in a single pass but I want to leave room to
    // create optimized instructions between these two steps, like
    // converting "+++" to inc(3) or "[-]" to set(0)

    let mut ifs = vec![];
    for i in 0..code.len() {
        match code[i] {
            Instruction::If(_) => { ifs.push(i); },
            Instruction::Fi(_) => {
                let if_idx = ifs.pop().unwrap();
                code[i] = Instruction::Fi(if_idx + 1); // jump to the first instruction after [
                code[if_idx] = Instruction::If(i + 1); // jump to the first instruction after ]
            }
            _ => {}
        }
    }

    assert!(ifs.is_empty());

    Program { code }
}
