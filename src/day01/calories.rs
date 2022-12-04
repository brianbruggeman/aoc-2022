use std::cmp::Reverse;
static DEFAULT_INPUT: &str = include_str!("input.txt");

pub fn calculate(input: Option<&str>, count: usize) -> usize {
    let data = match input {
        Some(v) => v,
        None => DEFAULT_INPUT,
    };
    let elves = parse(data);
    let mut elf_calories = Vec::new();
    for elf in elves.iter() {
        let mut calories = 0;
        for calorie_count in elf.split('\n') {
            calories += calorie_count
                .parse::<usize>()
                .expect("Could not parse calorie");
        }
        elf_calories.push(calories);
    }
    elf_calories.sort_by_key(|w| Reverse(*w));
    let calories = elf_calories
        .iter()
        .enumerate()
        .filter(|(idx, _)| idx < &count)
        .map(|(_, value)| value)
        .sum();
    calories
}

fn parse(input: &str) -> Vec<String> {
    let mut elves = Vec::new();
    let mut values = "".to_string();
    for line in input.split('\n') {
        if line.trim() == "" {
            elves.push(values);
            values = "".to_string();
        } else {
            match values.is_empty() {
                true => values = line.to_string(),
                false => values = format!("{values}\n{line}"),
            }
        }
    }
    elves
}

#[cfg(test)]
mod test {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case("a\nb\n", &["a\nb"])]
    #[case("a\nb\n\nc", &["a\nb", "c"])]
    #[case("a\nb\n\nc\nd", &["a\nb", "c\nd"])]
    pub fn test_parse(#[case] input: &str, #[case] output: &[&str]) {
        let result = parse(input);
        // let output = output.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        for (r, o) in result.iter().zip(output) {
            assert_eq!(r, o);
        }
    }

    #[rstest]
    #[case("1", 1, 0)]
    #[case("1\n", 1, 1)]
    #[case("1\n2\n\n4\n", 1, 4)]
    #[case("1\n5\n\n3\n", 1, 6)]
    #[case("1\n5\n\n3\n", 2, 9)]
    #[case("1\n5\n\n3\n\n1\n\n1\n\n1\n", 2, 9)]
    pub fn test_calculate(#[case] text: &str, #[case] count: usize, #[case] output: usize) {
        let result = calculate(Some(text), count);
        assert_eq!(result, output);
    }
}
