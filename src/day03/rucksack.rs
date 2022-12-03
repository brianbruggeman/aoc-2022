#[allow(non_camel_case_types)]
type priority = usize;

pub fn find_badge_priority(contents: &str) -> priority {
    let mut chunk = vec![];
    println!("Line count: {}", contents.trim().split('\n').collect::<Vec<_>>().len());
    let mut badges_priorities = 0;
    for (lineno, line) in contents.trim().split('\n').enumerate() {
        chunk.push(line);
        if lineno % 3 == 2 {
            let possible_badges = intersection(chunk[0], chunk[1]).into_iter().collect::<String>();
            let team_badge = intersection(&possible_badges, chunk[2]).into_iter().collect::<String>();
            let team_priority = get_priority(&team_badge);
            badges_priorities += team_priority;
            chunk.clear();
        }
    }
    badges_priorities
}

pub fn prioritize(contents: &str) -> priority {
    contents
        .split('\n')
        .map(|line| compartmentize(line))
        .map(|(compartment1, compartment2)| find_common_letter(compartment1, compartment2))
        .filter(|common_character| !common_character.is_empty())
        .map(|common_character| get_priority(common_character))
        .sum()
}

pub fn find_common_letter<'a>(a: &'a str, b: &'a str) -> &'a str {
    for (a_index, each_a) in a.chars().enumerate() {
        for (_index, each_b) in b.chars().enumerate() {
            if each_a == each_b {
                return &a[a_index..(a_index + 1)];
            }
        }
    }
    ""
}
pub fn intersection(a: &str, b: &str) -> Vec<char> {
    let mut common = vec![];
    for x in a.chars() {
        for y in b.chars() {
            if x == y && !common.contains(&x) {
                common.push(y)
            }
        }
    }
    common
}

pub fn find_common_badge<'a>(a: &'a str, b: &'a str, c: &'a str) -> &'a str {

    for (a_index, each_a) in a.chars().enumerate() {
        for (_index, each_b) in b.chars().enumerate() {
            for (_index, each_c) in c.chars().enumerate() {
                if each_a == each_b && each_a == each_c {
                    return &a[a_index..(a_index + 1)];
                }
            }
        }
    }
    ""
}

pub fn get_priority(letter: &str) -> priority {
    let lowercase_offset = 96;
    let uppercase_offset = 64;
    let alphabet_letter_count = 26;
    let num = letter.chars().take(1).last().unwrap() as priority;
    if num > lowercase_offset && num <= lowercase_offset + alphabet_letter_count {
        num - lowercase_offset
    } else {
        num - uppercase_offset + alphabet_letter_count
    }
}

pub fn compartmentize<'a>(line: &'a str) -> (&'a str, &'a str) {
    let split = line.len() / 2;
    (&line[..split], &line[split..])
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::*;

    #[rstest]
    #[case("ab", ("a", "b"))]
    #[case("abcd", ("ab", "cd"))]
    pub fn test_compartmentize(#[case] input: &str, #[case] expected: (&str, &str)) {
        let actual = compartmentize(input);
        assert_eq!(actual, expected)
    }

    #[rstest]
    #[case("a", 1)]
    #[case("b", 2)]
    #[case("z", 26)]
    #[case("A", 27)]
    #[case("B", 28)]
    #[case("Z", 52)]
    pub fn test_get_priority(#[case] input: &str, #[case] expected: usize) {
        let actual = get_priority(input);
        assert_eq!(actual, expected)
    }

    #[rstest]
    #[case("abc", "cde", "c")]
    #[case("abc", "def", "")]
    #[case("abc", "bcd", "b")]
    pub fn test_find_common(#[case] left: &str, #[case] right: &str, #[case] expected: &str) {
        let actual = find_common_letter(left, right);
        assert_eq!(actual, expected)
    }

    #[test]
    pub fn test_prioritize() {
        let input = textwrap::dedent(r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
            "#);

        let actual = prioritize(&input.trim());
        assert_eq!(actual, 157);
    }

    #[test]
    pub fn test_find_badge_priority() {
        let input = textwrap::dedent(r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
            "#);

        let actual = find_badge_priority(&input.trim());
        assert_eq!(actual, 70);
    }
}