#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_macros)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::comparison_chain)]
#![allow(clippy::nonminimal_bool)]
#![allow(clippy::neg_multiply)]
#![allow(dead_code)]
#![recursion_limit = "1024"]
use std::cmp::min;
use std::collections::{vec_deque, BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque};
use std::f64::consts::PI;
use std::mem::swap;
use std::println;
use std::process::exit;
use std::time::{Duration, Instant};

use itertools::{CombinationsWithReplacement, Itertools};
use num::{integer::Roots, traits::Pow, ToPrimitive};
use num_integer::{div_ceil, Integer};
use num_traits::{abs_sub, Float};
use permutohedron::Heap;
use proconio::input;
use proconio::marker::{Chars, Usize1};
use rand::seq::SliceRandom;
use rand::Rng;
use whiteread::parse_line;
// use std::io::*;
//
struct Input {
    D: usize,
    s: Vec<Vec<i64>>,
    c: Vec<i64>,
}

struct State {
    out: Vec<usize>,
    score: i64,
    ds: Vec<Vec<usize>>,
}

fn cost(a: usize, b: usize) -> i64 {
    let d = b - a;
    return (d * (d - 1) / 2) as i64;
}

impl State {
    fn new(input: &Input, out: Vec<usize>) -> State {
        let mut ds = vec![vec![]; 26];
        for d in 0..input.D {
            ds[out[d]].push(d + 1);
        }
        let score = compute_score(&input, &out);
        State { out, score, ds }
    }

    fn change(&mut self, input: &Input, d: usize, new_i: usize) {
        let old_i = self.out[d];
        let p = self.ds[old_i].iter().position(|a| *a == d + 1).unwrap();
        let prev = self.ds[old_i].get(p - 1).cloned().unwrap_or(0);
        let next = self.ds[old_i].get(p + 1).cloned().unwrap_or(input.D + 1);
        self.ds[old_i].remove(p);
        self.score += (cost(prev, d + 1) + cost(d + 1, next) - cost(prev, next)) * input.c[old_i];

        let p = self.ds[new_i]
            .iter()
            .position(|a| *a > d + 1)
            .unwrap_or(self.ds[new_i].len());
        let prev = self.ds[new_i].get(p - 1).cloned().unwrap_or(0);
        let next = self.ds[new_i].get(p).cloned().unwrap_or(input.D + 1);
        self.ds[new_i].insert(p, d + 1);

        self.score -= (cost(prev, d + 1) + cost(d + 1, next) - cost(prev, next)) * input.c[new_i];
        self.score += input.s[d][new_i] - input.s[d][old_i];
        self.out[d] = new_i;
    }
}

fn compute_score(input: &Input, out: &Vec<usize>) -> i64 {
    let mut score = 0;
    let mut last = vec![0; 26];
    for d in 0..out.len() {
        last[out[d]] = d + 1;
        for i in 0..26 {
            score -= (d + 1 - last[i]) as i64 * input.c[i];
        }
        score += input.s[d][out[d]];
    }
    return score;
}

fn evaluate(input: &Input, out: &Vec<usize>, k: usize) -> i64 {
    // out + k日何もしないで経過した場合のスコア
    let mut score = 0;
    let mut last = vec![0; 26];
    for d in 0..out.len() {
        last[out[d]] = d + 1;
        for i in 0..26 {
            score -= (d + 1 - last[i]) as i64 * input.c[i];
        }
        score += input.s[d][out[d]];
    }
    for d in out.len()..(out.len() + k).min(input.D) {
        for i in 0..26 {
            score -= (d + 1 - last[i]) as i64 * input.c[i];
        }
    }
    return score;
}

fn greedy(input: &Input) -> Vec<usize> {
    let mut out = vec![];
    for _ in 0..input.D {
        let mut max_score = -(1 << 60);
        let mut best_i = 1 << 30;
        for i in 0..26 {
            out.push(i);
            let score = compute_score(&input, &out);
            // let eval = evaluate(&input, &out, k);
            // eprintln!("eval = {}(k = {})", eval, k);
            if max_score < score {
                max_score = score;
                best_i = i;
                eprintln!("update max score: {}", max_score);
            }
            out.pop();
        }
        out.push(best_i);
        // println!("day {},  out len = {}", day, out.len());
    }
    return out;
}
fn greedy2(input: &Input) -> Vec<usize> {
    let mut best_out = vec![];
    let mut best_score = -(1 << 60);
    for k in 0..26 {
        let mut out = vec![];
        for _ in 0..input.D {
            let mut max_score = -(1 << 60);
            let mut best_i = 1 << 30;
            for i in 0..26 {
                out.push(i);
                // let score = compute_score(&input, &out);
                let eval = evaluate(&input, &out, k);
                // eprintln!("eval = {}(k = {})", eval, k);
                if max_score < eval {
                    max_score = eval;
                    best_i = i;
                    // eprintln!("update max score: {}", max_score);
                }
                out.pop();
            }
            out.push(best_i);
            // println!("day {},  out len = {}", day, out.len());
        }
        // let score = evaluate(&input, &out, k);
        // eprintln!("out \n{:?}", out);
        let score = compute_score(&input, &out);
        if score > best_score {
            best_score = score;
            best_out = out.clone();

            // eprintln!("best score: {} k = {}", best_score, k);
            // break;
        }
    }
    return best_out;
}

fn local_search(input: &Input) -> Vec<usize> {
    const TL: u128 = 1900;
    let mut rng = rand_pcg::Pcg64Mcg::new(809482);
    let mut out = (0..input.D)
        .map(|_| rng.gen_range(0, 26))
        .collect::<Vec<_>>();
    let start = Instant::now();
    let mut end = start.elapsed();
    let mut score = compute_score(&input, &out);
    while end.as_millis() < TL {
        let d = rng.gen_range(0, input.D);
        let q = rng.gen_range(0, 26);
        let old = out[d];
        out[d] = q;
        let new_score = compute_score(&input, &out);
        if new_score > score {
            score = new_score;
            // eprintln!("new score {}", score);
        } else {
            out[d] = old;
        }
        end = start.elapsed();
    }
    return out;
}

fn local_search2(input: &Input) -> Vec<usize> {
    const TL: u128 = 1900;
    let mut rng = rand_pcg::Pcg64Mcg::new(809482);
    let mut out = (0..input.D)
        .map(|_| rng.gen_range(0, 26))
        .collect::<Vec<_>>();
    let start = Instant::now();
    let mut end = start.elapsed();
    let mut score = compute_score(&input, &out);
    while end.as_millis() < TL {
        if rng.gen_bool(0.5) {
            let d = rng.gen_range(0, input.D);
            let q = rng.gen_range(0, 26);
            let old = out[d];
            out[d] = q;
            let new_score = compute_score(&input, &out);
            if new_score > score {
                score = new_score;
                // eprintln!("new score {}", score);
            } else {
                out[d] = old;
            }
        } else {
            let d1 = rng.gen_range(0, input.D - 1);
            let d2 = rng.gen_range(d1 + 1, (d1 + 16).min(input.D));
            out.swap(d1, d2);
            let new_score = compute_score(&input, &out);
            if score < new_score {
                score = new_score;
            } else {
                out.swap(d1, d2);
            }
        }
        end = start.elapsed();
    }
    return out;
}

fn simulated_annealing(input: &Input) -> Vec<usize> {
    const T0: f64 = 2e3;
    const T1: f64 = 6e2;
    const TL: f64 = 1.95;
    let start = Instant::now();
    let mut rng = rand_pcg::Pcg64Mcg::new(812);
    let mut state = State::new(&input, (0..input.D).map(|_| rng.gen_range(0, 26)).collect());
    let mut cnt = 0;
    let mut T = T0;
    let mut best = state.score;
    let mut best_out = state.out.clone();
    loop {
        cnt += 1;
        if cnt % 100 == 0 {
            let end = start.elapsed().as_secs_f64();
            let t = end / TL;
            if t >= 1.0 {
                break;
            }
            T = T0.powf(1.0 - t) * T1.powf(t);
        }
        let old_score = state.score;
        if rng.gen_bool(0.5) {
            // eprintln!("annealing");
            let d = rng.gen_range(0, input.D);
            let old = state.out[d];
            state.change(input, d, rng.gen_range(0, 26));
            if old_score > state.score
                && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T))
            {
                state.change(input, d, old);
            }
        } else {
            // eprintln!("swap");
            let d1 = rng.gen_range(0, input.D - 1);
            let d2 = rng.gen_range(d1 + 1, (d1 + 16).min(input.D));
            let (a, b) = (state.out[d1], state.out[d2]);
            state.change(input, d1, b);
            state.change(input, d2, a);
            if old_score > state.score
                && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T))
            {
                state.change(input, d1, a);
                state.change(input, d2, b);
            }
        }
        if best < state.score {
            best = state.score;
            best_out = state.out.clone();
        }
    }

    return best_out;
}

fn simulated_annealing2(input: &Input) -> Vec<usize> {
    const T0: f64 = 2e3;
    const T1: f64 = 6e2;
    const TL: f64 = 1.95;
    let start = Instant::now();
    let mut rng = rand_pcg::Pcg64Mcg::new(812);
    let mut state = State::new(&input, (0..input.D).map(|_| rng.gen_range(0, 26)).collect());
    let mut cnt = 0;
    let mut T = T0;
    let mut best = state.score;
    let mut best_out = state.out.clone();
    loop {
        cnt += 1;
        if cnt % 100 == 0 {
            let end = start.elapsed().as_secs_f64();
            let t = end / TL;
            if t >= 1.0 {
                break;
            }
            T = T0.powf(1.0 - t) * T1.powf(t);
        }
        let old_score = state.score;
        if rng.gen_bool(0.5) {
            // eprintln!("annealing");
            let d = rng.gen_range(0, input.D);
            let old = state.out[d];
            state.change(input, d, rng.gen_range(0, 26));
            if old_score > state.score
                && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T))
            {
                state.change(input, d, old);
            }
        } else {
            // eprintln!("swap");
            let d1 = rng.gen_range(0, input.D - 2);
            let d2 = rng.gen_range(d1 + 1, (d1 + 16).min(input.D - 1));
            let d3 = rng.gen_range(d2 + 1, (d2 + 16).min(input.D));
            let idxes = vec![state.out[d1], state.out[d2], state.out[d3]];
            let mut chances = vec![d1, d2, d3];
            chances.shuffle(&mut rng);
            for (i, d) in chances.iter().enumerate() {
                state.change(input, *d, idxes[i]);
            }
            if old_score > state.score
                && !rng.gen_bool(f64::exp((state.score - old_score) as f64 / T))
            {
                state.change(input, d1, idxes[0]);
                state.change(input, d2, idxes[1]);
                state.change(input, d3, idxes[2]);
            }
        }
        if best < state.score {
            best = state.score;
            best_out = state.out.clone();
        }
    }

    return best_out;
}
fn solve(input: &Input) {
    // let out = greedy(&input);
    // let out = greedy2(&input);
    // let out = local_search(&input);
    // let out = local_search2(&input);
    // let out = simulated_annealing(&input);
    let out = simulated_annealing2(&input);

    for a in out.iter() {
        println!("{}", a + 1);
    }
}
fn main() {
    // input! {N:usize}
    // for _ in 0..N {
    //     solve();
    // }
    input! {
        D:usize,
        c:[i64; 26],
        s:[[i64; 26]; D]
    }
    let input = Input { D, s, c };
    solve(&input);
}
