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

use itertools::{CombinationsWithReplacement, Itertools, Tuples};
use num::{integer::Roots, traits::Pow, ToPrimitive};
use num_integer::{div_ceil, Integer};
use num_traits::{abs_sub, Float};
use permutohedron::Heap;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use whiteread::parse_line;
// use std::io::*;

#[derive(Clone, Debug)]
struct S {
    i: usize,
    j: usize,
}

#[derive(Clone, Debug)]
struct Input {
    s: S,
    T: Vec<Vec<usize>>,
    P: Vec<Vec<usize>>,
}

fn is_ok(i: isize, j: isize) -> bool {
    (0 <= i && i < 50 && 0 <= j && j < 50)
}

fn greedy(input: &Input) -> Vec<char> {
    let mut out = vec![];
    let mut s = input.s.clone();
    let mut visited = HashSet::new();
    visited.insert(input.T[s.i][s.j]);
    let di = [1, 0, -1, 0];
    let dj = [0, 1, 0, -1];
    let directions = ['D', 'R', 'U', 'L'];
    loop {
        let mut best_point = 0;
        let mut best_direction = 0;
        let mut ns = s.clone();
        // eprintln!("ns {:?}", ns);
        for i in 0..4 {
            let ni = s.i as isize + di[i];
            let nj = s.j as isize + dj[i];
            if !(0 <= ni && ni < 50 && 0 <= nj && nj < 50) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            // eprintln!("ni = {}, nj = {}, P = {}", ni, nj, input.P[ni][nj]);
            if visited.contains(&input.T[ni][nj]) {
                continue;
            }
            if best_point < input.P[ni][nj] {
                best_point = input.P[ni][nj];
                best_direction = i;
                ns.i = ni;
                ns.j = nj;
            }
        }
        // eprintln!("");
        if best_point == 0 {
            break;
        }
        visited.insert(input.T[ns.i][ns.j]);
        s = ns.clone();
        out.push(directions[best_direction]);
    }

    return out;
}

fn greedy2(input: &Input) -> Vec<char> {
    let mut out = vec![];
    let mut s = input.s.clone();
    let mut visited = HashSet::new();
    visited.insert(input.T[s.i][s.j]);
    let di = [1, 0, -1, 0];
    let dj = [0, 1, 0, -1];
    let directions = ['D', 'R', 'U', 'L'];
    loop {
        let mut best_point = 0;
        let mut best_direction = 0;
        let mut ns = s.clone();
        let mut is_forward = false;
        // eprintln!("ns {:?}", ns);
        for i in 0..4 {
            let ni = s.i as isize + di[i];
            let nj = s.j as isize + dj[i];
            if !(0 <= ni && ni < 50 && 0 <= nj && nj < 50) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            // eprintln!("ni = {}, nj = {}, P = {}", ni, nj, input.P[ni][nj]);
            if visited.contains(&input.T[ni][nj]) {
                continue;
            }
            let mut point = input.P[ni][nj];
            for j in 0..4 {
                let nni = ni as isize + di[j];
                let nnj = nj as isize + dj[j];
                if !(0 <= nni && nni < 50 && 0 <= nnj && nnj < 50) {
                    continue;
                }
                let nni = nni as usize;
                let nnj = nnj as usize;
                // eprintln!("ni = {}, nj = {}, P = {}", ni, nj, input.P[ni][nj]);
                if visited.contains(&input.T[nni][nnj]) {
                    continue;
                }

                if input.T[ni][nj] == input.T[nni][nnj] {
                    continue;
                }
                // eprintln!("ns = {:?}, nni = ({}, {})", ns, nni, nnj);
                if nni == ns.i && nnj == ns.j {
                    continue;
                }
                point += input.P[nni][nnj];
                if best_point < point {
                    best_point = point;
                    best_direction = i;
                    ns.i = ni;
                    ns.j = nj;
                    is_forward = true;
                }
            }
        }
        // eprintln!("");
        if !is_forward {
            break;
        }
        visited.insert(input.T[ns.i][ns.j]);
        s = ns.clone();
        out.push(directions[best_direction]);
    }

    return out;
}

fn bfs(input: &Input) -> Vec<char> {
    let mut out = vec![];
    let mut dists = vec![vec![-1; 50]; 50];
    dists[input.s.i][input.s.j] = 0;
    let mut que = VecDeque::new();
    que.push_back(input.s.clone());
    let di = [1, 0, -1, 0];
    let dj = [0, 1, 0, -1];
    let dir = ['D', 'R', 'U', 'L'];
    let rev_dir = ['U', 'L', 'D', 'R'];
    let mut visited = HashSet::new();
    visited.insert(input.T[input.s.i][input.s.j]);
    while !que.is_empty() {
        let s = que.pop_back().unwrap();
        for i in 0..4 {
            let ni = s.i as isize + di[i];
            let nj = s.j as isize + dj[i];
            if !is_ok(ni, nj) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if input.T[ni][nj] == input.T[s.i][s.j] {
                continue;
            }
            if visited.contains(&input.T[ni][nj]) {
                continue;
            }
            if dists[ni][nj] < dists[s.i][s.j] + 1 {
                dists[ni][nj] = dists[s.i][s.j] + 1;
                // eprintln!("dists max: {}", dists[ni][nj]);
                que.push_back(S { i: ni, j: nj });
                visited.insert(input.T[ni][nj]);
            }
        }
    }
    let mut s = S { i: 0, j: 0 };
    for i in 0..50 {
        for j in 0..50 {
            if dists[i][j] > dists[s.i][s.j] {
                s.i = i;
                s.j = j;
            }
        }
    }
    // for d in dists.iter() {
    //     eprintln!("{:?}", d);
    // }
    // return out;

    // eprintln!("keiro hukugen");
    // eprintln!("{:?}, {}", s, input.P[s.i][s.j]);
    let mut visited = HashSet::new();
    visited.insert(input.T[s.i][s.j]);
    while input.s.i != s.i || input.s.j != s.j {
        for i in 0..4 {
            let ni = s.i as isize + di[i];
            let nj = s.j as isize + dj[i];
            if !is_ok(ni, nj) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            // eprintln!("{:?}, ({}, {})", s, ni, nj);
            // eprintln!("{}, {}", dists[s.i][s.j], dists[ni][nj]);
            if dists[ni][nj] + 1 == dists[s.i][s.j] {
                s.i = ni;
                s.j = nj;
                out.push(rev_dir[i]);
                visited.insert(input.T[s.i][s.j]);
                // eprintln!("{:?}", s);
                break;
            }
        }
        // eprintln!("{:?}", out);
    }
    out.reverse();
    return out;
}

fn bfs_greedy(input: &Input) -> Vec<char> {
    let mut out = vec![];
    let mut dists = vec![vec![-1; 50]; 50];
    dists[input.s.i][input.s.j] = 0;
    let mut que = VecDeque::new();
    que.push_back(input.s.clone());
    let di = [1, 0, -1, 0];
    let dj = [0, 1, 0, -1];
    let dir = ['D', 'R', 'U', 'L'];
    let rev_dir = ['U', 'L', 'D', 'R'];
    let mut visited = HashSet::new();
    visited.insert(input.T[input.s.i][input.s.j]);
    while !que.is_empty() {
        let s = que.pop_back().unwrap();
        for i in 0..4 {
            let ni = s.i as isize + di[i];
            let nj = s.j as isize + dj[i];
            if !is_ok(ni, nj) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if input.T[ni][nj] == input.T[s.i][s.j] {
                continue;
            }
            if visited.contains(&input.T[ni][nj]) {
                continue;
            }
            if dists[ni][nj] < dists[s.i][s.j] + 1 {
                dists[ni][nj] = dists[s.i][s.j] + 1;
                // eprintln!("dists max: {}", dists[ni][nj]);
                que.push_back(S { i: ni, j: nj });
                visited.insert(input.T[ni][nj]);
            }
        }
    }
    let mut s = S { i: 0, j: 0 };
    for i in 0..50 {
        for j in 0..50 {
            if dists[i][j] > dists[s.i][s.j] {
                s.i = i;
                s.j = j;
            }
        }
    }
    // for d in dists.iter() {
    //     eprintln!("{:?}", d);
    // }
    // return out;

    // eprintln!("keiro hukugen");
    // eprintln!("{:?}, {}", s, input.P[s.i][s.j]);
    let mut visited = HashSet::new();
    let mut ns = s.clone();
    visited.insert(input.T[s.i][s.j]);
    while input.s.i != s.i || input.s.j != s.j {
        for i in 0..4 {
            let ni = s.i as isize + di[i];
            let nj = s.j as isize + dj[i];
            if !is_ok(ni, nj) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            // eprintln!("{:?}, ({}, {})", s, ni, nj);
            // eprintln!("{}, {}", dists[s.i][s.j], dists[ni][nj]);
            if dists[ni][nj] + 1 == dists[s.i][s.j] {
                s.i = ni;
                s.j = nj;
                out.push(rev_dir[i]);
                visited.insert(input.T[s.i][s.j]);
                // eprintln!("{:?}", s);
                break;
            }
        }
        // eprintln!("{:?}", out);
    }
    out.reverse();
    // eprintln!("{:?}", ns);
    loop {
        let mut is_changed = false;
        let mut best_point = 0;
        let mut best_dir = 0;
        for i in 0..4 {
            let ni = ns.i as isize + di[i];
            let nj = ns.j as isize + dj[i];
            if !is_ok(ni, nj) {
                continue;
            }
            let ni = ni as usize;
            let nj = nj as usize;
            if visited.contains(&input.T[ni][nj]) {
                continue;
            }
            if best_point < input.P[ni][nj] {
                best_point = input.P[ni][nj];
                best_dir = i;
                is_changed = true;
            }
        }
        if !is_changed {
            break;
        }
        ns.i = (ns.i as isize + di[best_dir]) as usize;
        ns.j = (ns.j as isize + dj[best_dir]) as usize;
        // eprintln!("{:?}, {}", ns, input.T[ns.i][ns.j]);
        // eprintln!("{}", visited.contains(&input.T[ns.i][ns.j]));
        out.push(dir[best_dir]);
        visited.insert(input.T[ns.i][ns.j]);
    }
    return out;
}

fn dfs(input: &Input) -> Vec<char> {
    let mut out = vec![];
    let mut tnum = 0;
    for i in 0..50 {
        for j in 0..50 {
            tnum = tnum.max(input.T[i][j]);
        }
    }
    let mut visited = vec![false; tnum];
    visited[input.T[input.s.i][input.s.j]] = true;
    return out;
}

fn solve() {
    input! {
        si:usize, sj:usize,
        T:[[usize; 50]; 50],
        P:[[usize; 50]; 50]
    }
    let input = Input {
        s: S { i: si, j: sj },
        T,
        P,
    };
    // let out = greedy(&input);
    // let out = greedy2(&input);
    // let out = bfs(&input);
    let out = bfs_greedy(&input);
    println!("{}", out.iter().join(""));
}

fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    solve();
}
