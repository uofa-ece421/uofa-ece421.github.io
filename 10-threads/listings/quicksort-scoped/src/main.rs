extern crate rand;

use std::env;
use rand::Rng;
use std::time::Instant;
use std::convert::TryInto;

const MILLION: usize = 1024*1024;

fn quicksort<T: PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    quicksort(lo);
    quicksort(hi);
}

// ANCHOR: parallel
fn parallel_quicksort<T: PartialOrd + std::marker::Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    if v.len() <= 2048 {
        quicksort(v);
        return;
    }

    crossbeam::scope(|scope| {
        let mid = partition(v);
        let (lo, hi) = v.split_at_mut(mid);
        scope.spawn(move |_| parallel_quicksort(lo));
        scope.spawn(move |_| parallel_quicksort(hi));
    }).expect("thread spawn failed");
}
// ANCHOR_END: parallel

fn partition<T: PartialOrd>(v: &mut [T]) -> usize {
    let hi = v.len() - 1;
    let lo = 0;
    let pivot = hi;

    let mut i = 0;
    for j in lo..hi {
        if v[j] <= v[pivot] {
            v.swap(i, j);
            i += 1;
        }
    }
    v.swap(i, pivot);
    i
}

fn initialize_data(len: usize) -> Vec<u32> {
    let mut data = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();

    data.resize(len, 0);
    for i in 0..len {
        data[i] = rng.gen_range(0..len).try_into().unwrap();
    }
    data
}

// ANCHOR: bench
fn reinitialize_data(v: &mut [u32]) {
    let mut rng = rand::thread_rng();
    let len = v.len();
    for i in 0..len {
        v[i] = rng.gen_range(0..len).try_into().unwrap();
    }
}

fn bench<F: FnOnce(&mut [u32])>(sort: F, data: &mut [u32]) -> u64 {
    reinitialize_data(data);

    let start = Instant::now();
    sort(data);
    let elapsed = Instant::now() - start;

    if !data.windows(2).all(|w| w[0] <= w[1]) {
        println!("sort failed!");
    }

    u64::from(elapsed.subsec_micros()) + elapsed.as_secs()*1000000u64
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // First argument is the vector length
    let len = match args.get(1) {
        Some(slen) => match slen.parse::<usize>() {
            Ok(num) => num,
            Err(e) => {
                println!("ignoring bad length: {}", e);
                MILLION as usize
            }
        },
        None => MILLION as usize,
    };

    let mut data = initialize_data(len);

    let mut par_mean: f64 = 0.0;
    for run in 0..10 {
        let elapsed = bench(parallel_quicksort, &mut data);
        println!("parallel {}: {} usec", run, elapsed);
        par_mean += elapsed as f64;
    }
    par_mean /= 10.0;
    println!("parallel mean: {} usec", par_mean);

    let mut seq_mean: f64 = 0.0;
    for run in 0..10 {
        let elapsed = bench(quicksort, &mut data);
        println!("sequential {}: {}", run, elapsed);
        seq_mean += elapsed as f64;
    }
    seq_mean /= 10.0;
    println!("sequential mean: {} usec", seq_mean);

    println!("speedup: {}", seq_mean/par_mean);
}
// ANCHOR_END: bench
