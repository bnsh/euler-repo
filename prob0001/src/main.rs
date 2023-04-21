// vim: set expandtab shiftwidth=4 tabstop=4:

/*
 * Project Euler: 0001: Multiples of 3 or 5:
 * If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
 * Find the sum of all the multiples of 3 or 5 below 1000.
 */

fn main() {
    let mut sum = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i
        }
    }
    println!("{sum}")
}
