mod calories;

pub fn run() {
    println!("Day 1:");
    println!("    Part 1: {} calories", calories::calculate(None, 1));
    println!("    Part 2: {} calories", calories::calculate(None, 3));
}
