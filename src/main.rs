use turing::*;

fn main() {
    let description = std::fs::read_to_string("turing.tm").unwrap();

    let mut tm: TuringMachine = description.parse().unwrap();

    let iterations = 500;

    for _ in 0..iterations {
        tm.step();
    }

    tm.tape().iter().step_by(2).for_each(|c| print!("{c}"));
    println!();
    //println!("{}", tm.tape());
}
