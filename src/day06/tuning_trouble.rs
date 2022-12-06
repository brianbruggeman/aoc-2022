use std::collections::HashSet;

pub fn detect_start(data_stream: &str, packet_header_size: usize) -> usize {
    let packet_index = packet_header_size - 1;
    for (i, _c) in data_stream.chars().enumerate() {
        if i < packet_index {
            continue;
        }
        if entry_is_unique(&data_stream[i - packet_index..=i]) {
            return i + 1;
        }
    }
    0
}

fn entry_is_unique(data_entry: &str) -> bool {
    let data: HashSet<char> = data_entry.chars().collect();
    data.len() == data_entry.chars().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("a", true)]
    #[case("aa", false)]
    #[case("aba", false)]
    #[case("abc", true)]
    fn test_entry_is_unique(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(entry_is_unique(input), expected)
    }
}
