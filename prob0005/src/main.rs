// vim: expandtab shiftwidth=4 tabstop=4:

/*
 * Problem 00005: Smallest multiple
 *
 * 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
 * What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?
 */

/*
 * 2 -> 2
 * 3 -> 3
 * 4 -> 2 2
 * 5 -> 5
 * 6 -> 2 3
 * 7 -> 7
 * 8 -> 2 2 2
 * 9 -> 3 3
 * 10 -> 2 5
 *
 * 2 3 5 7 11 13 ...
 * 3 2 1 1
 * represents
 * 2^3 * 3^2 * 5^1 * 7^1  = 8 * 9 * 5 * 7 = 72 * 35 = (70 + 2) * (30 + 5) = 2100 + 60 + 350 + 10 = 2450 + 70 = 2520
 */

use std::collections::HashMap;
use std::cmp::max;

fn factorize(n: u64) -> HashMap<u64, u8> {
	let mut factors : HashMap<u64, u8> = HashMap::new();

	let mut sofar = n;
	let mut cand = 2;
	while cand * cand <= sofar {
		if sofar % cand == 0 {
			sofar = sofar / cand;
			factors.insert(cand, 1 + *(factors.get(&cand).unwrap_or(&0)));
			cand = 1;
		}
		cand += 1;
	}
	factors.insert(sofar, 1 + *(factors.get(&sofar).unwrap_or(&0)));

	return factors;
}

fn merge(cumfactors: &mut HashMap<u64, u8>, factors: &HashMap<u64, u8>) {
	for (factor, cand_count) in factors.iter() {
		let current_count = *(cumfactors.get(&factor).unwrap_or(&0));
		cumfactors.insert(*factor, max(current_count, *cand_count));
	}
}

fn unravel(factors: HashMap<u64, u8>) -> u64 {
	let mut val: u64 = 1;
	for (factor, exponent) in factors.iter() {
		for _ in 0..(*exponent) {
			val = val * (*factor);
		}
	}
	return val;
}

fn smallest(n: u64) -> u64 {
	let mut cumfactors : HashMap<u64, u8> = HashMap::new();
	for val in 2..(n+1) {
		let factors = factorize(val);

		merge(&mut cumfactors, &factors);
	}
	let res = unravel(cumfactors);
	return res;
}

fn main() {
	let q = smallest(20);
	println!("{q}");
}
