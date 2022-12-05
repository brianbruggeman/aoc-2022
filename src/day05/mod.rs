mod supply_stacks;

pub fn run() {
    let items = include_str!("input.txt");
    println!("Day 5:");
    println!("    Crane 9000: {:?}", supply_stacks::arrange_crates_9000(items));
    println!("    Crane 9001: {:?}", supply_stacks::arrange_crates_9001(items));
}
