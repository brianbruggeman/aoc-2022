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
    println!("    Tail Position Count(1): {:?}", rope_bridge::count_tail_position(items, 2));
    println!("    Tail Position Count(10): {:?}", rope_bridge::count_tail_position(items, 10));
}

#[cfg(test)]
mod tests {
    use super::rope_bridge::*;
    use rstest::rstest;

    #[rstest]
    #[case::simple_tail_example1("example1", 2, 13)]
    #[case::simple_tail_example2("example2", 2, 88)]
    #[case::long_tail_example1("example1", 10, 1)]
    #[case::long_tail_example2("example2", 10, 36)]
    fn test_count_tail_position(#[case] data_set: &str, #[case] knots: usize, #[case] expected: usize) {
        let data1 = include_str!("example.txt");
        let data2 = include_str!("example2.txt");
        let data_set = match data_set {
            "example1" => data1,
            "example2" => data2,
            _ => unreachable!(),
        };
        let actual = count_tail_position(data_set, knots);
        assert_eq!(actual, expected);
    }
}
