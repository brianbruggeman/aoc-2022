mod rope_bridge;

pub fn run(use_example: bool) {
    #[allow(unused_assignments)]
    let mut items = "";
    if use_example {
        items = include_str!("example.txt");
    } else {
        items = include_str!("input.txt");
    }
    println!("Day 9:");
    println!("    Tail Position Count: {:?}", rope_bridge::count_tail_position(items));
}

#[cfg(test)]
mod tests {
    use super::rope_bridge::*;

    #[test]
    fn test_count_tail_position() {
        let data = include_str!("example.txt");
        let actual = count_tail_position(data);
        assert_eq!(actual, 13);
    }
}
