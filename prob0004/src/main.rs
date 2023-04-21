// vim: expandtab shiftwidth=4 tabstop=4:

/*
 * Problem 00004: Largest palindrome product
 *
 * A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
 * Find the largest palindrome made from the product of two 3-digit numbers.
 */

/*
fn is_palindrome(inp: String) -> bool {
    let chars : Vec<char> = inp.chars().collect();
    let mut hyp: bool = true;

    for (start, chr) in chars.iter().enumerate() {
        let end = inp.len() - 1 - start;
        let lchr = chars[end];
        if lchr != *chr {
            hyp = false;
        }
    }
    hyp
}
 */

// Asked ChatGPT for a better version of is_palindrome and got:
fn is_palindrome_chatgpt(inp: String) -> bool {
    let reversed: String = inp.chars().rev().collect();
    inp == reversed
}

fn main() {
    let mut best_palindrome = (-1, -1, -1);

    for idx1 in 0..1000 {
        for idx2 in idx1..1000 {
            let prod = idx1 * idx2;
            if is_palindrome_chatgpt(prod.to_string()) {
                if best_palindrome.0 == -1 || best_palindrome.0 < prod {
                    best_palindrome = (prod, idx1, idx2);
                }
            }
        }
    }
    println!("{0}", best_palindrome.0);
}
