mod camp_cleaning;

pub fn run() {
    let items = include_str!("input.txt");
    println!("Day 4:");
    println!("    Fully Overlap: {}", camp_cleaning::count_fully_contained(items));
    println!("    Partial Overlap: {}", camp_cleaning::count_partial_overlap(items));
}
