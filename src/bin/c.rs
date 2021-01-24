use proconio::input;
use std::cmp;


fn main() {
    input! {
        n: usize,
        a: [i32; 2_i32.pow(n as u32)]
    }

    let size = a.len();

    let (vec_lh, vec_rh) = a.split_at(size / 2);
    // println!("{:?} {:?}",vec_lh,vec_rh);

    let (index_lh, max_lh) = find_max_with_index(vec_lh);
    let (index_rh, max_rh) = find_max_with_index(vec_rh);

    let ans = if max_lh > max_rh {index_rh + size / 2 + 1} else {index_lh + 1};

    println!("{}", ans)
}

fn find_max_with_index(x_vec: &[i32]) -> (usize, i32){
    let mut max: i32 = x_vec[0];
    let mut index: usize = 0;
    for (i, x) in x_vec.iter().enumerate(){
        if *x > max { index = i }
        max = cmp::max(*x, max);
    }

    return (index, max);
}