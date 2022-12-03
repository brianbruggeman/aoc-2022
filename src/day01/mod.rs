mod calories;

pub fn run() {
    println!("Day 1:");
    println!("    Maximum: {} calories", calories::calculate(None, 1));
    println!("      Max 3: {} calories", calories::calculate(None, 3));
}
