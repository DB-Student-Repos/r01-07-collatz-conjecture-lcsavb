pub fn collatz(mut n: u64) -> Option<u64> {
    let mut steps = 0;

    for step in 0.. {
        if n == 1 {
            return Some(steps);
        } else if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }

        steps = steps + 1;
    }

    Some(steps)


}