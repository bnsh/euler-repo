// vim: expandtab shiftwidth=4 tabstop=4:

/*
 * Problem 00009: Special Pythagorean triplet
 *
 * A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
 *
 * a^2 + b^2 = c^2
 * For example, 3^2 + 4^2 = 9 + 16 = 25 = 52.
 *
 * There exists exactly one Pythagorean triplet for which a + b + c = 1000.
 * Find the product abc.
 * https://en.wikipedia.org/wiki/Formulas_for_generating_Pythagorean_triples#Pythagorean_triples_by_use_of_matrices_and_linear_transformations
 * 
 * *sigh* Overthought this. It doesn't say the triplet need be primitive. So, GCD(a, b, c) can be > 1
 */

fn main() {
    for a in 1..1000 {
        for b in (1+a)..1000 {
            let c2 = a * a + b * b;
            let c = (c2 as f32).sqrt() as u32;
            if (c2 == c * c) && (a + b + c) == 1000 {
                println!("({}, {}, {}): sum={}, prod={} zero={}", a, b, c, a+b+c, a*b*c, a*a+b*b-c*c)
            }
        }
    }
}
