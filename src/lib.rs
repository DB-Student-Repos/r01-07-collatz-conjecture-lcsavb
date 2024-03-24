pub fn collatz(n: u64) -> Option<u64> {

    if n == 0 {
        return None;
    }
    
    let mut steps = 0;
    let mut current = n;

    while current != 1 {
        if current % 2 == 0 {
            current /= 2;
        } else {
            current = current.checked_mul(3)?.checked_add(1)?;
        }
        steps += 1;
    }

    Some(steps)
}