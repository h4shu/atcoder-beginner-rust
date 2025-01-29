use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }
    let mut sum = 0;
    for c in s {
        if c == '1' {
            sum += 1;
        }
    }
    println!("{}", sum);
}
