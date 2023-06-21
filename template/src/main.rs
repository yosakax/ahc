#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
#![recursion_limit = "1024"]
use std::collections::{vec_deque, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::mem::swap;
use std::println;
use std::process::exit;

use itertools::{CombinationsWithReplacement, Itertools};
use num::{integer::Roots, traits::Pow, ToPrimitive};
use num_integer::{div_ceil, Integer};
use num_traits::{abs_sub, Float};
use permutohedron::Heap;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use whiteread::parse_line;
// use std::io::*;
//

fn bisect_left<T: std::cmp::PartialOrd>(lst: &Vec<T>, val: T) -> usize {
    let mut left = 0;
    let mut right = lst.len();

    while left < right {
        let mid = (left + right) / 2;
        if val <= lst[mid] {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    return left;
}

fn gcd<T: std::ops::Rem<Output = T> + std::cmp::PartialEq<i32> + Copy>(a: T, b: T) -> T {
    if a % b == 0 {
        return b;
    } else {
        return gcd(b, a % b);
    }
}

fn modpow(mut a: i64, mut p: i64, MOD: i64) -> i64 {
    // 繰り返し2乗法による高速べき乗mod
    let mut ret = 1;
    while p > 0 {
        if p % 2 == 1 {
            ret *= a;
        }
        a *= a;
        p >>= 1;
        a %= MOD;
        ret %= MOD;
    }
    return ret;
}

fn solve() {}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}
