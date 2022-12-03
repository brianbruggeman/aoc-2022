mod rucksack;

pub fn run() {
    let items = include_str!("input.txt");
    println!("Day 3:");
    println!("    Summed Priority: {}", rucksack::prioritize(items));
    println!("    Badges Priority: {}", rucksack::find_badge_priority(items));
}
