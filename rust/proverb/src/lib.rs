pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 1 {
        return format!("And all for the want of a {}.", list[0]);
    }
    if list.is_empty() {
        return String::new();
    }

    let mut result = String::from("");
    let first_item = list[0].clone();
    for pair in list.windows(2) {
        let first = pair[0];
        let second = pair[1];
        let proverb_line = format!("For want of a {} the {} was lost.\n", first, second);
        result.push_str(proverb_line.as_str());
    }
    let final_line = format!("And all for the want of a {}.", first_item);
    result.push_str(final_line.as_str());
    result
}
