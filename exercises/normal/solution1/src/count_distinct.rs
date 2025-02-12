pub fn new_count_distinct(input_str: &str) -> usize {
    let mut hash_set = HashSet::new();

    for str in input_str.split(",") {
          hash_set.insert(str);
    }
    hash_set.len()
}
