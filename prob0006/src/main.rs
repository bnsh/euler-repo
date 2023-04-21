// vim: expandtab shiftwidth=4 tabstop=4:

/*
 * Problem 00006: Sum Square Difference
 *
The sum of the squares of the first ten natural numbers is,
	  1^2 + 2^2 + 3^2 + 4^2 + 5^2 + 6^2 + 7^2 + 8^2 + 9^2 + 10^2
	=   1 +   4 +   9 +  16 +  25 +  36 +  49 +  64 +  81 +  100
	= 385

The square of the sum of the first ten natural numbers is,
	(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10)^2
	= 55^2
	= 3025

Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 
	3025 - 385 = 2640
.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.
 */

fn main() {
	let n : i64 = 100;
	let sum_squares : i64 = (0..=n).map(|x| x*x).sum();
	let square_sum : i64 = ((0..=n).sum::<i64>()).pow(2);

	println!("{}", square_sum - sum_squares);
}
