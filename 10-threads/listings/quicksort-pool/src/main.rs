use std::env;
use rand::Rng;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Mutex;

mod tpool;

const MILLION: u32 = 256; 
static PAR_SEQ_THRESHOLD: AtomicU32 = AtomicU32::new(32);

// ANCHOR: pool
use tpool::ThreadPool;
use lazy_static::lazy_static;

lazy_static! {
    static ref THREAD_POOL: Mutex<ThreadPool> = Mutex::new(ThreadPool::new(8));
}

fn parallel_quicksort<T: PartialOrd + std::marker::Send + std::marker::Sync>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    if v.len() <= PAR_SEQ_THRESHOLD.load(Ordering::SeqCst) as usize {
        quicksort(v);
        return;
    }

    let mid = partition(v);
    // let (lo, hi) = v.split_at_mut(mid);
    let len = v.len();

    let w1 = THREAD_POOL.lock().unwrap().execute(move || { println!("exec: mid {}", mid); } );
    let w2 = THREAD_POOL.lock().unwrap().execute(move || { println!("exec: len {}", len); });
    THREAD_POOL.lock().unwrap().wait(w1);
    THREAD_POOL.lock().unwrap().wait(w2);
}
// ANCHOR_END: pool

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

fn initialize_data(len: usize) -> Vec<u32> {
    let mut data = Vec::with_capacity(len);
    let mut rng = rand::thread_rng();

    data.resize(len, 0);
    for i in 0..len {
        data[i] = rng.gen_range(0..len).try_into().unwrap();
    }
    data
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

    println!("len {}", len);
    let mut v = initialize_data(len);
    parallel_quicksort(&mut v);

    THREAD_POOL.lock().unwrap().drain();
}
