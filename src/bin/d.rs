use proconio::input;
use std::cmp;

fn main() {
    input! {
        (n, c_max): (usize, i64),
        abc : [(usize, usize, i64); n]
    }
    let mut events = vec![];
    for i in 0..n{
        events.push((abc[i].0 - 1, abc[i].2));
        events.push((abc[i].1, - abc[i].2));
    }
    events.sort();

    let mut ans = 0;
    let mut fee = 0i64;
    let mut x_prev = 0;

    for (x, y) in events {
        if x != x_prev{
            ans += cmp::min(fee, c_max) * (x - x_prev) as i64;
            x_prev = x;
        }
        fee += y;
    }

    println!("{}", ans);
}
