mod no_space;

pub fn run(use_example: bool) {
    #[allow(unused_assignments)]
    let mut items = "";
    if use_example {
        items = include_str!("example.txt");
    } else {
        items = include_str!("input.txt");
    }
    println!("Day 7:");
    println!("    Sum of folder sizes below 100k: {:?}", no_space::find_good_deletion_candidates(items));
    println!("    Folder size to delete: {:?}", no_space::folder_to_delete(items));
}

#[cfg(test)]
mod tests {
    use super::no_space::{find_good_deletion_candidates, folder_to_delete};
    
    #[test]
    pub fn test_find_good_deletion_candidates() {
        let data = include_str!("example.txt");
        let actual = find_good_deletion_candidates(data);
        assert_eq!(actual, 95_437);
    }
    
    #[test]
    pub fn test_folder_to_delete() {
        let data = include_str!("example.txt");
        let actual = folder_to_delete(data);
        assert_eq!(actual, 24_933_642);
    }
}