mod tree_top;

pub fn run(use_example: bool) {
    #[allow(unused_assignments)]
    let mut items = "";
    if use_example {
        items = include_str!("example.txt");
    } else {
        items = include_str!("input.txt");
    }
    println!("Day 8:");
    println!("    Trees visible: {:?}", tree_top::find_trees_visible(items));
    println!("    Scenic score: {:?}", tree_top::find_highest_scenic_score(items));
}

#[cfg(test)]
mod tests {
    use super::tree_top::*;

    #[test]
    pub fn test_find_trees_visible() {
        let data = include_str!("example.txt");
        let actual = find_trees_visible(data);
        assert_eq!(actual, 21);
    }

    #[test]
    pub fn test_find_highest_scenic_score() {
        let data = include_str!("example.txt");
        let actual = find_highest_scenic_score(data);
        assert_eq!(actual, 8);
    }
}
