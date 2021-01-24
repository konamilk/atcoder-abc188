use proconio::input;

fn main() {
    input! {
        n: u32,
        a: [i32; n],
        b: [i32; n]
    }

    // println!("{} {:?} {:?}", n, a, b)

    let mut product: i32 = 0;
    for i in 0..n {
        product += a[i as usize] * b[i as usize];
    }

    let ans = if product == 0 {"Yes"} else {"No"};

    println!("{}", ans)
}
