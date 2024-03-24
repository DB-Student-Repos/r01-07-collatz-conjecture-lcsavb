pub fn collatz(n: u64) -> Option<u64> {
    let mut steps = 0;

    if n == 0 {
        return None;
    }
    
    let mut current = n;

    while current != 1 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            if current > (std::u64::MAX - 1) / 3 {
                return None;
            }
            current = 3 * current + 1;
        }
        steps += 1;
    }

    Some(steps)
}