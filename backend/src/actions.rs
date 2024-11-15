/// Checks if the given number is prime
pub fn is_prime(num: u64) -> bool {
    if num < 2 {
        return false;
    }
    for i in 2..=((num as f64).sqrt() as u64) {
        if num % i == 0 {
            return false;
        }
    }
    true
}

/// Finds all prime numbers less than the given number `n`
/// using the Sieve of Eratosthenes algorithm.
pub fn generate_primes(n: u64) -> Vec<u64> {
    if n < 2 {
        return vec![];
    }

    let mut is_prime = vec![true; n as usize];
    is_prime[0] = false; // 0 is not prime
    is_prime[1] = false; // 1 is not prime

    for i in 2..=((n as f64).sqrt() as usize) {
        if is_prime[i] {
            for j in (i * i..n as usize).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &prime)| prime)
        .map(|(i, _)| i as u64)
        .collect()
}

/// Finds the median prime number(s) from a given list of primes.
pub fn median_primes(primes: &[u64]) -> Vec<u64> {
    let len = primes.len();
    if len == 0 {
        return vec![];
    } else if len % 2 == 1 {
        vec![primes[len / 2]]
    } else {
        vec![primes[len / 2 - 1], primes[len / 2]]
    }
}
