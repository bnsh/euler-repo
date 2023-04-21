// vim: expandtab shiftwidth=4 tabstop=4:

/*
 * Problem 0003: Largest prime factor
 * The prime factors of 13195 are 5, 7, 13 and 29.
 * What is the largest prime factor of the number 600851475143 ?
 */

fn prime_factors(inp: i64) -> Vec<i64> {
    let mut primes: Vec<i64> = Vec::new();
    let mut candidate = 2;
    let mut num = inp;

    while candidate * candidate < num {
        if num % candidate == 0 {
            primes.push(candidate);
            num /= candidate;
            candidate = 1;
        }
        candidate += 1;
    }
    if num != 1 {
        primes.push(num);
    }
    primes
}

fn main() {
    let primes = prime_factors(600851475143);
    let largest = primes[primes.len()-1];
    println!("{largest}");
}
