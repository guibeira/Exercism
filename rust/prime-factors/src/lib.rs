pub fn factors(n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    let mut divisor = 2;
    let mut num = n;

    while num > 1 {
        if num.is_multiple_of(divisor) {
            factors.push(divisor);
            num /= divisor;
        } else {
            divisor += 1;
        }
    }

    factors
}
