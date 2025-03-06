use proconio::input;

fn main() {
    input! {
        n: i32,
        a: [i32; n],
    }
    let mut sum = 0;
    let mut flag = true;
    loop {
        let d = 2_i32.pow(sum + 1);
        for i in a.iter() {
            if i % d != 0 {
                flag = false;
                break;
            }
        }
        if !flag {
            break;
        }
        sum += 1;
    }
    println!("{}", sum);
}
