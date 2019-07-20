pub fn collatz(n: u64) -> Option<u64> {
    let mut steps: i32 = 0;
    let mut sum: i32 = n as i32;
    if n < 1 {
        return None;
    }
    while sum != 1 {
        if sum % 2 == 0 {
            sum /= 2;
        } else {
            sum = (sum * 3) + 1;
        }
        steps += 1;;
    }
    Some(steps as u64)
}
