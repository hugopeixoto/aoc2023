fn main() {
    let code = std::fs::read_to_string(std::env::args().last().unwrap()).unwrap();
    let program = aoc2023::brainfuck::compile(&code);

    program.run();
}
