pub fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

pub fn pick(n: u32, k: u32) -> u32 {
    factorial(n) / factorial(n - k)
}

pub fn choose(n: u32, k: u32) -> u32 {
    if n < k {
        0
    } else {
        factorial(n) / (factorial(k) * factorial(n - k))
    }
}
