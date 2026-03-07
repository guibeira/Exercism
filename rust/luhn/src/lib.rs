pub fn is_valid(code: &str) -> bool {
    for c in code.chars() {
        if c.is_whitespace() || (c.is_ascii() && c.is_numeric()) {
            continue;
        } else {
            return false;
        }
    }

    let mut only_digits = code
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_string().parse::<i32>().unwrap()) 
        .collect::<Vec<i32>>();
    only_digits.reverse();

    if only_digits.len() == 1 && only_digits[0] == 0 {
        return false;
    }

    for i in 1..only_digits.len() + 1 {
        if i % 2 == 0 {
            let position_value= only_digits[i - 1];
            let mut double_value = position_value * 2;
            if double_value > 9 {
                double_value -= 9;
            } 
            only_digits[i-1] = double_value;
        } 
    }

    let sum: i32 = only_digits.iter().sum();
    sum % 10 == 0
}
