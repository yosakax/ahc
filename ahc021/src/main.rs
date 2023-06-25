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
use std::process::exit;
use std::time::Instant;
use std::{println, vec};

use itertools::{CombinationsWithReplacement, Itertools, PutBack, Tuples};
use num::{integer::Roots, traits::Pow, ToPrimitive};
use num_integer::{div_ceil, Integer};
use num_traits::{abs_sub, Float, WrappingAdd};
use permutohedron::Heap;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use rand::seq::SliceRandom;
use whiteread::parse_line;
// use std::io::*;

#[derive(Clone, Debug)]
struct Input {
    N: usize,
    K: usize,
    B: Vec<Vec<usize>>,
}

#[derive(Clone, Debug)]
struct Output {
    out: Vec<(usize, usize, usize, usize)>,
    B: Vec<Vec<usize>>,
}

fn evaluate(B: &Vec<Vec<usize>>, K: usize) -> usize {
    let mut err_cnt = 0;
    let dx = [1, 1];
    let dy = [0, 1];
    for i in 0..B.len() - 1 {
        for j in 0..B[i].len() {
            for k in 0..dx.len() {
                let nx = i + dx[k];
                let ny = j + dy[k];
                if ny >= B[i + 1].len() {
                    continue;
                }
                if B[i][j] > B[nx][ny] {
                    err_cnt += 1;
                }
            }
        }
    }
    let mut score = 0;
    if err_cnt == 0 {
        score += 100000 - 5 * K;
    } else {
        eprintln!("error count :{}", err_cnt);
        score += 50000 - 50 * err_cnt;
    }
    return score;
}
fn is_ok(B: &Vec<Vec<usize>>) -> bool {
    let dx = [1, 1];
    let dy = [0, 1];
    for i in 0..B.len() - 1 {
        for j in 0..B[i].len() {
            for k in 0..dx.len() {
                let nx = i + dx[k];
                let ny = j + dy[k];
                if ny >= B[i + 1].len() {
                    continue;
                }
                if B[i][j] > B[nx][ny] {
                    return false;
                }
            }
        }
    }
    return true;
}

fn is_ok_single(i: usize, j: usize, B: &Vec<Vec<usize>>) -> bool {
    if i == B.len() - 1 {
        return false;
    }
    if B[i].len() == j + 1 {
        return B[i][j] < B[i + 1][j];
    } else {
        return B[i][j] < B[i + 1][j] && B[i][j] < B[i + 1][j + 1];
    }
}

fn greedy(input: &Input, start: &Instant) -> Output {
    // let mut pairs = VecDeque::new();
    let mut B = input.B.clone();
    let mut out = vec![];
    // let dx = [0, 0, 1, 1];
    // let dy = [-1, 1, 0, 1];
    let dx = [1, 1, -1];
    let dy = [0, 1, -1];
    let mut cnt = 0;
    const TL: u128 = 1900;
    'outer: loop {
        let end = start.elapsed();
        // eprintln!("{}", end.as_millis());
        if end.as_millis() >= TL {
            break;
        }
        let mut is_changed = false;
        for i in 0..input.N {
            for j in 0..B[i].len() {
                let (x, y) = (i, j);
                for k in 0..dx.len() {
                    let nx = x as isize + dx[k];
                    let ny = y as isize + dy[k];

                    if !(0 <= nx && nx < input.N as isize && 0 <= ny && ny < B[i].len() as isize) {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if x < nx {
                        if B[x][y] > B[nx][ny] && !is_ok_single(x, y, &B) {
                            is_changed = true;
                            out.push((x, y, nx, ny));
                            let old = B[x][y];
                            B[x][y] = B[nx][ny];
                            B[nx][ny] = old;
                            cnt += 1;
                            if cnt == input.K {
                                break 'outer;
                            }
                            let end = start.elapsed();
                            if end.as_millis() >= TL {
                                break 'outer;
                            }
                        }
                    } else {
                        // 上方向確認
                        if B[x][y] < B[nx][ny] {
                            is_changed = true;
                            out.push((x, y, nx, ny));
                            let old = B[x][y];
                            B[x][y] = B[nx][ny];
                            B[nx][ny] = old;
                            cnt += 1;
                            if cnt == input.K {
                                break 'outer;
                            }
                            let end = start.elapsed();
                            if end.as_millis() >= TL {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        // if is_changed {
        //     eprintln!("score: {}, ops={}", evaluate(&B, out.len()), out.len());
        // }
        if !is_changed {
            break;
        }
    }
    // let mut out = vec![vec![]];
    eprintln!("score: {}, ops={}", evaluate(&B, out.len()), out.len());
    let out = Output { out, B };
    return out;
}

fn greedy_tateyoko(input: &Input, start: &Instant) -> Output {
    // 横のソート→縦の順でやってく
    let mut B = input.B.clone();
    let mut out = vec![];
    let mut rng = rand_pcg::Mcg128Xsl64::new(812709);
    // let mut dx = [1, 1, -1];
    // let mut dy = [0, 1, -1];
    let mut d = [(1, 0), (1, 1), (-1, -1)];
    const TL: u128 = 1900;
    // 一旦ソートしておく
    for i in 0..input.N {
        for j in 0..B[i].len() {
            let (x, y) = (i, j);
            // let dx = [0, 1, 1, -1];
            // let dy = [-1, 0, 1, -1];
            d.shuffle(&mut rng);

            for k in 0..d.len() {
                let nx = x as isize + d[k].0;
                let ny = y as isize + d[k].1;

                if !(0 <= nx && nx < input.N as isize && 0 <= ny && ny < B[i].len() as isize - 1) {
                    continue;
                }
                let nx = nx as usize;
                let ny = ny as usize;
                if x < nx {
                    if B[x][y] > B[nx][ny] {
                        out.push((x, y, nx, ny));
                        let old = B[x][y];
                        B[x][y] = B[nx][ny];
                        B[nx][ny] = old;
                    }
                } else {
                    // 上方向確認
                    if B[x][y] < B[nx][ny] {
                        out.push((x, y, nx, ny));
                        let old = B[x][y];
                        B[x][y] = B[nx][ny];
                        B[nx][ny] = old;
                    }
                }
            }
        }
    }
    for j in 0..input.N {
        for i in 0..B.len() / 2 {
            if j > i {
                continue;
            }
            if B[i][j] > B[i + 1][j] {
                out.push((i, j, i + 1, j));
                let old = B[i][j];
                B[i][j] = B[i + 1][j];
                B[i + 1][j] = old;
            }
        }
    }

    for j in 0..input.N {
        for i in ((B.len() / 2)..B.len() - 1).rev() {
            if j >= i {
                continue;
            }

            if B[i][j] < B[i - 1][j] {
                out.push((i, j, i - 1, j));
                let old = B[i][j];
                B[i][j] = B[i - 1][j];
                B[i - 1][j] = old;
            }
        }
    }
    for i in 0..B.len() {
        for j in 0..B[i].len() / 2 {
            if B[i][j] > B[i][j + 1] {
                out.push((i, j, i, j + 1));
                let old = B[i][j];
                B[i][j] = B[i][j + 1];
                B[i][j + 1] = old;
            }
        }
    }
    for i in 0..B.len() / 2 {
        for j in (B[i].len() / 2..B[i].len() - 1).rev() {
            if B[i][j] < B[i][j - 1] {
                out.push((i, j, i, j - 1));
                let old = B[i][j];
                B[i][j] = B[i][j - 1];
                B[i][j - 1] = old;
            }
        }
    }

    let dx = [1, 1, -1];
    let dy = [0, 1, -1];

    'outer: loop {
        let end = start.elapsed();
        if end.as_millis() >= TL {
            break;
        }
        let mut is_changed = false;
        for j in 0..input.N {
            for i in 0..B.len() - 1 {
                for i2 in 0..B.len() - 1 {
                    if j > i2 {
                        continue;
                    }
                    if B[i2][j] > B[i2 + 1][j] && !is_ok_single(i2, j, &B) {
                        out.push((i2, j, i2 + 1, j));
                        let old = B[i2][j];
                        B[i2][j] = B[i2 + 1][j];
                        B[i2 + 1][j] = old;
                        is_changed = true;
                        if out.len() == input.K {
                            break 'outer;
                        }
                    }
                    // if !is_ok_single(i2, j, &B) {
                    //     eprintln!("{}, {}", i2, j);
                    // }
                }
            }
        }
        for i in 0..input.N {
            for j in 0..B[i].len() {
                let (x, y) = (i, j);
                for k in 0..dx.len() {
                    let nx = x as isize + dx[k];
                    let ny = y as isize + dy[k];

                    if !(0 <= nx && nx < input.N as isize && 0 <= ny && ny < B[i].len() as isize) {
                        continue;
                    }
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if x < nx {
                        if B[x][y] > B[nx][ny] && !is_ok_single(x, y, &B) {
                            // if B[x][y] > B[nx][ny] {
                            is_changed = true;
                            out.push((x, y, nx, ny));
                            let old = B[x][y];
                            B[x][y] = B[nx][ny];
                            B[nx][ny] = old;
                            if out.len() == input.K {
                                break 'outer;
                            }
                            let end = start.elapsed();
                            if end.as_millis() >= TL {
                                break 'outer;
                            }
                        }
                    } else {
                        // 上方向確認
                        if B[x][y] < B[nx][ny] {
                            is_changed = true;
                            out.push((x, y, nx, ny));
                            let old = B[x][y];
                            B[x][y] = B[nx][ny];
                            B[nx][ny] = old;
                            if out.len() == input.K {
                                break 'outer;
                            }
                            let end = start.elapsed();
                            if end.as_millis() >= TL {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
        for i in 0..B.len() / 2 {
            for j in 0..B[i].len() - 1 {
                for j2 in 0..B[i].len() - 1 {
                    let mut flag = false;
                    // 左のほうが大きければswap
                    if i == B.len() - 1 && j2 == B[i].len() - 1 {
                        flag = B[i][j2] > B[i][j2 + 1];
                    } else {
                        flag = B[i][j2] > B[i][j2 + 1] && !is_ok_single(i, j2, &B);
                    }
                    if flag {
                        out.push((i, j2, i, j2 + 1));
                        let old = B[i][j2];
                        B[i][j2] = B[i][j2 + 1];
                        B[i][j2 + 1] = old;
                        is_changed = true;
                        if out.len() == input.K {
                            break 'outer;
                        }
                    }
                    // if !is_ok_single(i, j2, &B) {
                    //     eprintln!("{}, {}, {}", i, j2, B[i][j2]);
                    // }
                }
            }
        }
        for i in (B.len() / 2..B.len()).rev() {
            for j in 0..B[i].len() - 1 {
                for j2 in (B.len() / 2..B[i].len() - 1).rev() {
                    let mut flag = false;
                    // 右のほうが大きければswap
                    if i == B.len() - 1 && j2 == B[i].len() - 1 {
                        flag = B[i][j2] > B[i][j2 - 1];
                    } else {
                        flag = B[i][j2] > B[i][j2 - 1] && !is_ok_single(i, j2, &B);
                    }
                    if flag {
                        out.push((i, j2, i, j2 - 1));
                        let old = B[i][j2];
                        B[i][j2] = B[i][j2 - 1];
                        B[i][j2 - 1] = old;
                        is_changed = true;
                        if out.len() == input.K {
                            break 'outer;
                        }
                    }
                    // if !is_ok_single(i, j2, &B) {
                    //     eprintln!("{}, {}, {}", i, j2, B[i][j2]);
                    // }
                }
            }
        }
        if !is_changed {
            break;
        }
    }
    let out = Output { out, B };
    eprintln!(
        "score: {}, ops={}",
        evaluate(&out.B, out.out.len()),
        out.out.len()
    );

    return out;
}

fn greedy_minsort(input: &Input, start: &Instant) -> Output {
    const TL: u128 = 1900;

    let mut heap = BinaryHeap::new();
    let mut mp = HashMap::new();
    let mut B = input.B.clone();
    for i in 0..B.len() {
        for j in 0..B[i].len() {
            mp.insert(B[i][j], (i, j));
            if i > 0 {
                heap.push((B[i][j] as isize * -1, i, j));
            }
        }
    }
    let mut out = vec![];
    loop {
        let &v = mp.keys().max().unwrap();
        let &(i, j) = mp.get(&v).unwrap();
        if i == 29 || (j == 0 || j == 29) {
            break;
        }
        if j < 15 {
            out.push((i + 1, j, i, j));
            let &(i2, j2) = mp.get(&B[i + 1][j]).unwrap();
            mp.insert(v, (i2, j2));
            mp.insert(B[i2][j2], (i, j));
            let old = B[i][j];
            B[i][j] = B[i2][j2];
            B[i2][j2] = old;
        } else {
            out.push((i + 1, j + 1, i, j));
            let &(i2, j2) = mp.get(&B[i + 1][j + 1]).unwrap();
            mp.insert(v, (i2, j2));
            mp.insert(B[i2][j2], (i, j));
            let old = B[i][j];
            B[i][j] = B[i2][j2];
            B[i2][j2] = old;
        }
    }
    loop {
        let &v = mp.keys().max().unwrap();
        let &(i, j) = mp.get(&v).unwrap();
        if j == 0 || j == 29 {
            break;
        }
        if j < 15 {
            out.push((i, j - 1, i, j));
            let &(i2, j2) = mp.get(&B[i][j - 1]).unwrap();
            mp.insert(v, (i2, j2));
            mp.insert(B[i2][j2], (i, j));
            let old = B[i][j];
            B[i][j] = B[i2][j2];
            B[i2][j2] = old;
        } else {
            out.push((i, j + 1, i, j));
            let &(i2, j2) = mp.get(&B[i][j + 1]).unwrap();
            mp.insert(v, (i2, j2));
            mp.insert(B[i2][j2], (i, j));
            let old = B[i][j];
            B[i][j] = B[i2][j2];
            B[i2][j2] = old;
        }
    }

    // eprintln!("{:?}", out);
    let &vmax = mp.keys().max().unwrap();
    for v in 0..vmax {
        if !mp.contains_key(&v) {
            break;
        }
        let end = start.elapsed();
        // eprintln!("elapsed: {}", end.as_millis());
        if end.as_millis() >= TL {
            break;
        }
        // eprintln!("v = {}", v);
        while out.len() < input.K {
            let end = start.elapsed();
            if end.as_millis() >= TL {
                break;
            }
            // let (v, i, j) = heap.pop().unwrap();
            // let v = (-v) as usize;
            let &(i, j) = mp.get(&v).unwrap();
            let mut is_changed = false;
            if i == 0 {
                break;
            }

            // eprintln!("v = {} i = {} j = {}", v, i, j);
            if j == 0 {
                if B[i - 1][j] > B[i][j] {
                    is_changed = true;
                    out.push((i - 1, j, i, j));
                    let &(i2, j2) = mp.get(&B[i - 1][j]).unwrap();
                    mp.insert(v, (i2, j2));
                    mp.insert(B[i2][j2], (i, j));
                    let old = B[i][j];
                    B[i][j] = B[i - 1][j];
                    B[i - 1][j] = old;
                    // if i - 1 > 0 {
                    //     heap.push((B[i - 1][j] as isize * -1, i - 1, j));
                    // }
                }
            } else if j == i {
                if B[i - 1][j - 1] > B[i][j] {
                    is_changed = true;
                    out.push((i - 1, j - 1, i, j));
                    let &(i2, j2) = mp.get(&B[i - 1][j - 1]).unwrap();
                    mp.insert(v, (i2, j2));
                    mp.insert(B[i2][j2], (i, j));
                    let old = B[i][j];
                    B[i][j] = B[i - 1][j - 1];
                    B[i - 1][j - 1] = old;
                    // if i - 1 > 0 {
                    //     heap.push((B[i - 1][j - 1] as isize * -1, i - 1, j - 1));
                    // }
                }
            } else {
                if B[i - 1][j - 1] > B[i - 1][j] {
                    if B[i][j] < B[i - 1][j - 1] {
                        is_changed = true;
                        out.push((i - 1, j - 1, i, j));
                        let &(i2, j2) = mp.get(&B[i - 1][j - 1]).unwrap();
                        mp.insert(v, (i2, j2));
                        mp.insert(B[i2][j2], (i, j));
                        let old = B[i][j];
                        B[i][j] = B[i - 1][j - 1];
                        B[i - 1][j - 1] = old;
                        // if i - 1 > 0 {
                        //     heap.push((B[i - 1][j] as isize * -1, i - 1, j - 1));
                        // }
                    }
                } else {
                    if B[i - 1][j] > B[i][j] {
                        is_changed = true;
                        out.push((i - 1, j, i, j));
                        let &(i2, j2) = mp.get(&B[i - 1][j]).unwrap();
                        mp.insert(v, (i2, j2));
                        mp.insert(B[i2][j2], (i, j));
                        let old = B[i][j];
                        B[i][j] = B[i - 1][j];
                        B[i - 1][j] = old;
                        // if i - 1 > 0 {
                        //     heap.push((B[i - 1][j] as isize * -1, i - 1, j));
                        // }
                    }
                }
            }
            if !is_changed {
                break;
            }
            // eprintln!("vmid = {}, len = {}", v, out.len());
        }
        // eprintln!("vend = {}", v);
    }
    let out = Output { out, B };
    eprintln!(
        "score: {}, ops={}",
        evaluate(&out.B, out.out.len()),
        out.out.len()
    );

    return out;
}

// ----------------------------------------------------

fn solve() {
    let mut B = vec![];
    for i in 1..31 as usize {
        input! {
            b:[usize; i]
        }
        B.push(b);
    }
    let start = Instant::now();
    let input = Input { N: 30, K: 10000, B };
    // let out = greedy(&input, &start);
    // let out = greedy2(&input, &start);
    // let out = greedy_fixed(&input, &start);
    // let out = greedy_yokotate(&input, &start);
    // let out = greedy_tateyoko(&input, &start);
    let out = greedy_minsort(&input, &start);
    println!("{}", out.out.len());
    for v in out.out.iter() {
        println!("{} {} {} {}", v.0, v.1, v.2, v.3);
    }
    // println!("{:?}", out);
    // println!("{}", out.iter().join(""));
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}
