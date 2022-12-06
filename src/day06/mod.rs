mod tuning_trouble;

pub fn run() {
    let items = include_str!("input.txt");
    println!("Day 6:");
    println!("    Packet start: {:?}", tuning_trouble::detect_start(items, 4));
    println!("    Message start: {:?}", tuning_trouble::detect_start(items, 14));
}
