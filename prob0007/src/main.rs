// vim: expandtab shiftwidth=4 tabstop=4:

/*
 * Problem 00007: 10001st prime
 *
 * By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
 *
 * What is the 10 001st prime number?
 */

fn sieve() -> Vec::<usize> {
    /*
     * https://en.wikipedia.org/wiki/Prime_number_theorem
     * Pi(n) = 10001 = n / Log[n]
     * So... Newton's method?
     * f(n)  = n / Log[n] - 10001 = n (Log[n])^-1 - 10001
     * f'(n) = (Log[n])^-1 + n (-1)(Log[n])^-2 * 1/n
     *       = (Log[n])^-1 - n/(n Log[n]^2)
     *       = (1/Log[n]) - 1/(Log[n]^2)
     *       = (Log[n] - 1) / Log[n]^2
     * x, y = x, f(x)
     * f(x0) = f'(x0) x + b
     *
     * f'(x0)  x0     + b == f(x0)
     * f'(x0) (x0+dx) + b == 0
     *
     * f'(x0)(x0 - x0 - dx) == f(x0)
     * (-dx) = f(x0) / f'(x0)
     * dx = -f(x0) / f'(x0)
     * Well... All this is cool, but beyond my ken for now
     * in Rust. One day. For now, I'm just solving for n with
     * Mathematica: Solve[n/Log[n] == 10001, n]
     * Actually, it wouldn't "solve" for that. But,
     * 131072 / Log[131072] =~ 11123.3. So, let's try a sieve
     * of that size.
     */
    let mut primes: Vec::<usize> = Vec::new();

    let mut isprime: [bool; 131072] = [true; 131072];

    isprime[0] = false;
    isprime[1] = false;
    for cand in 2..131072 {
        if isprime[cand] {
            primes.push(cand);
            for idx in ((cand*2)..131072).step_by(cand) {
                isprime[idx] = false;
            }
        }
    }

    return primes;
}

fn main() {
    let primes = sieve();
    println!("{}", primes.get(10000).unwrap());
}
