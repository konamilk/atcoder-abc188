use proconio::{fastout, input};
use num::abs;

#[fastout]
fn main() {
    input! {
        x: i32,
        y: i32
    }
    let ans = if abs(x-y) < 3 {"Yes"} else {"No"};
    println!("{}", ans)
}
