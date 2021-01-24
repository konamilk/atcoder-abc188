use proconio::input;
// use std::cmp;

fn main() {
    input! {
        (n, prime): (usize, i64)
    }

    let mut a: Vec<usize> = Vec::new();
    let mut b: Vec<usize> = Vec::new();
    let mut c: Vec<i64> = Vec::new();

    for _i in 0..n{
        input! {
            (a_tmp, b_tmp, c_tmp): (usize, usize, i64)
        }
        a.push(a_tmp);
        b.push(b_tmp);
        c.push(c_tmp);
    }

    println!("{:?} {:?}", n, prime);
    println!("{:?} {:?} {:?}", a, b, c);
}
