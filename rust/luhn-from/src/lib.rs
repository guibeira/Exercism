pub struct Luhn(Vec<i32>);

impl Luhn {
    pub fn is_valid(&self) -> bool {
        let only_digits = &mut self.0.clone();
        if only_digits.len() == 1 && only_digits[0] == 0 {
            return false;
        }

        for i in 1..only_digits.len() + 1 {
            if i % 2 == 0 {
                let position_value = only_digits[i - 1];
                let mut double_value = position_value * 2;
                if double_value > 9 {
                    double_value -= 9;
                }
                only_digits[i - 1] = double_value;
            }
        }

        let sum: i32 = only_digits.iter().sum();
        sum % 10 == 0
    }
}


impl<T: ToString> From<T> for Luhn {
    fn from(input: T) -> Self {
        for i in input.to_string().chars() {
            if !i.is_whitespace() && (!i.is_ascii() || !i.is_numeric()) {
                return Luhn(vec![0]);
            }
        }
        let mut only_digits = input
            .to_string()
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_digit(10).unwrap_or(0) as i32)
            .collect::<Vec<i32>>();
        only_digits.reverse();
        Luhn(only_digits)
    }
}
