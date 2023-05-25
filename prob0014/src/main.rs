// vim: expandtab shiftwidth=4 tabstop=4:

/*
 * Problem 0014: Longest Collatz Sequence
 *
 * The following iterative sequence is defined for the set of positive integers:
 * 
 * n → n/2 (n is even)
 * n → 3n + 1 (n is odd)
 * 
 * Using the rule above and starting with 13, we generate the following sequence:
 * 
 * 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
 * It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms.
 * Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
 * 
 * Which starting number, under one million, produces the longest chain?
 * 
 * NOTE: Once the chain starts the terms are allowed to go above one million.
 */

fn collatz(n: u64) -> u64 {
    let mut val = n;
    let mut count = 1;
    while val != 1 {
        if val % 2 == 0 { val = val / 2; }
        else { val = 3 * val + 1; }
        count += 1;
    }
    count
}

fn main() {
    let mut best_length = 0;
    let mut best_idx = 0;
    for idx in 1..1000000 {
        let length = collatz(idx);
        if length > best_length {
            best_length = length;
            best_idx = idx;
        }
    }
    println!("{}", best_idx);
}
