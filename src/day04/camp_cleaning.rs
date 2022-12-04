pub fn count_fully_contained(pairs: &str) -> usize {
    pairs
        .lines()
        .filter(ignore_empty)
        .map(split_line)
        .map(calculate_contained)
        .sum()
}

pub fn count_partial_overlap(pairs: &str) -> usize {
    pairs
        .lines()
        .filter(ignore_empty)
        .map(split_line)
        .map(calculate_overlap)
        .sum()
}

pub fn ignore_empty(line: &&str) -> bool {
    !line.trim().is_empty()
}

pub fn split_line(line: &str) -> [&str; 2] {
    let index = line
        .chars()
        .position(|c| c == ',')
        .expect("Line is not valid");
    [&line[..index], &line[index + 1..]]
}

pub fn split_range(elf_line: &str) -> (usize, usize) {
    let index = elf_line
        .chars()
        .position(|c| c == '-')
        .expect("Elf line is not valid");
    let elf_min = &elf_line[..index]
        .parse::<usize>()
        .expect("Range is not valid");
    let elf_max = &elf_line[index + 1..]
        .parse::<usize>()
        .expect("Range is not valid");
    (*elf_min, *elf_max)
}

pub fn calculate_overlap(elves: [&str; 2]) -> usize {
    let (left_elf_min, left_elf_max) = split_range(elves[0]);
    let (right_elf_min, right_elf_max) = split_range(elves[1]);
    let left_elf_range = left_elf_min..=left_elf_max;
    let right_elf_range = right_elf_min..=right_elf_max;
    if (left_elf_range.contains(&right_elf_min) || left_elf_range.contains(&right_elf_max)) || (right_elf_range.contains(&left_elf_min) || right_elf_range.contains(&left_elf_max)) {
        return 1;
    }
    0
}

pub fn calculate_contained(elves: [&str; 2]) -> usize {
    let (left_elf_min, left_elf_max) = split_range(elves[0]);
    let (right_elf_min, right_elf_max) = split_range(elves[1]);
    let left_elf_range = left_elf_min..=left_elf_max;
    let right_elf_range = right_elf_min..=right_elf_max;
    if (left_elf_range.contains(&right_elf_min) && left_elf_range.contains(&right_elf_max)) || (right_elf_range.contains(&left_elf_min) && right_elf_range.contains(&left_elf_max)) {
        return 1;
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    const EXAMPLE: &str = r#"
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    "#;

    #[rstest]
    #[case(EXAMPLE, 2)]
    fn test_count_fully_contained(#[case] test_case: &str, #[case] expected: usize) {
        let input = textwrap::dedent(test_case);
        let actual = count_fully_contained(&input);
        assert_eq!(actual, expected)
    }

    #[rstest]
    #[case("2-4,6-8", 0)]
    #[case("2-3,4-5", 0)]
    #[case("5-7,7-9", 0)]
    #[case("2-8,3-7", 1)]
    #[case("6-6,4-6", 1)]
    #[case("2-6,4-8", 0)]
    fn test_calculate_contained(#[case] test_case: &str, #[case] expected: usize) {
        let elves = split_line(test_case);
        let actual = calculate_contained(elves);
        assert_eq!(actual, expected)
    }
}
