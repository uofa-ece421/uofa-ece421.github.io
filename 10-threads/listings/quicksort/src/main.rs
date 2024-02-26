extern crate rand;

use std::env;
use std::thread;
use rand::Rng;
use std::time::Instant;
use std::convert::TryInto;

// ANCHOR: sequential
fn quicksort<T: PartialOrd>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);
    quicksort(lo);
    quicksort(hi);
}

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
// ANCHOR_END: sequential

// ANCHOR: parallel
fn parallel_quicksort<T: PartialOrd + std::marker::Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    if v.len() < 2048 { // At some point it won't pay to start more threads
        quicksort(v);
        return;
    }

    let mid = partition(v);
    let (lo, hi) = v.split_at_mut(mid);

    let lo_handle = thread::spawn(move || { parallel_quicksort(lo) });
    let hi_handle = thread::spawn(move || { parallel_quicksort(hi) });

    lo_handle.join().unwrap();
    hi_handle.join().unwrap();
}
// ANCHOR_END: parallel

fn initialize_data(len: usize) -> Vec<u32> {
    let mut data = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();

    data.resize(len, 0);
    for i in 0..len {
        data[i] = rng.gen_range(0..len).try_into().unwrap();
    }
    data
}

fn reinitialize_data(v: &mut [u32]) {
    let mut rng = rand::thread_rng();
    let len = v.len();
    for i in 0..len {
        v[i] = rng.gen_range(0..len).try_into().unwrap();
    }
}
    
fn main() {
    let args: Vec<String> = env::args().collect();

    let len = match args.get(1) {
        Some(slen) => match slen.parse::<usize>() {
            Ok(num) => num,
            Err(e) => {
                println!("bad length: {}", e);
                return;
            }
        },
        None => {
            println!("Using default length {}", 1000000);
            1000000 as usize
        }
    };
        
    let mut data = initialize_data(len);

    let start = Instant::now();
    parallel_quicksort(&mut data);
    let elapsed = Instant::now() - start;

    let parallel_usec = u64::from(elapsed.subsec_micros()) + elapsed.as_secs()*1000000u64;
    println!("sorted {} values in parallel in {} usec", len, parallel_usec);  

    if !data.windows(2).all(|w| w[0] <= w[1]) {
        println!("sort failed!");
    }

    reinitialize_data(&mut data);

    let start = Instant::now();
    quicksort(&mut data);
    let elapsed = Instant::now() - start;

    let sequential_usec = u64::from(elapsed.subsec_micros()) + elapsed.as_secs()*1000000u64;
    println!("sorted {} values in {} usec", len, sequential_usec);  
    
    if !data.windows(2).all(|w| w[0] <= w[1]) {
        println!("sort failed!");
    }

    println!("speedup: {}", (sequential_usec as f64)/(parallel_usec as f64));
}
