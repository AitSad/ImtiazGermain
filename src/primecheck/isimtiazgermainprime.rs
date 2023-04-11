fn isimtiazgermainprime(n: u64) -> bool {
    // Check if n is a Sophie Germain prime
    let p = (n - 1) / 2;
    if !is_prime(n) || !is_prime(p) || !is_prime(2 * p + 1) {
        return false;
    }

    // Check if 2p + 1 is composite
    let m = 2 * p + 1;
    if is_prime(m) {
        return false;
    }

    // Check if 2(2p + 1) + 1 is prime
    let q = 2 * m + 1;
    is_prime(q)
}

// Helper function to check if a number is prime
fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    for i in 2..=n / 2 {
        if n % i == 0 {
            return false;
        }
    }

    true
}
