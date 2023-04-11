fn is_germain_prime(n: u64) -> bool {
    // Check if n is a prime number
    if !is_prime(n) {
        return false;
    }

    // Check if 2n + 1 is a prime number
    let m = 2 * n + 1;
    is_prime(m)
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
