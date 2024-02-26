use std::env;
use rand::Rng;
use std::time::Instant;
use std::sync::atomic::{AtomicU32, Ordering};

// ANCHOR: instrument
const MILLION: u32 = 1024*1024;

static THREAD_COUNT: AtomicU32 = AtomicU32::new(0);
static PAR_SEQ_THRESHOLD: AtomicU32 = AtomicU32::new(2048);

fn parallel_quicksort<T: PartialOrd + std::marker::Send>(v: &mut [T]) {
    if v.len() <= 1 {
        return;
    }

    if v.len() <= PAR_SEQ_THRESHOLD.load(Ordering::SeqCst) as usize {
        quicksort(v);
        return;
    }

    THREAD_COUNT.fetch_add(2, Ordering::SeqCst);
    
    crossbeam::scope(|scope| {
        let mid = partition(v);
        let (lo, hi) = v.split_at_mut(mid);
        scope.spawn(move |_| parallel_quicksort(lo));
        scope.spawn(move |_| parallel_quicksort(hi));
    }).expect("thread spawn failed");
}
// ANCHOR_END: instrument

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

    // Second argument is the parallel/sequential threshold
    match args.get(2) {
        Some(val) => match val.parse::<u32>() {
            Ok(t) => {
                PAR_SEQ_THRESHOLD.store(t, Ordering::SeqCst);
            },
            Err(e) => println!("ignoring bad value: {}", e),
        },
        None => (),
    };
    println!("Using len {}, par/seq threshold {}", len, PAR_SEQ_THRESHOLD.load(Ordering::SeqCst));
        
    let mut data = initialize_data(len);

    let mut par_mean: f64 = 0.0;
    for run in 0..10 {
        let elapsed = bench(parallel_quicksort, &mut data);
        let tcount = THREAD_COUNT.swap(0, Ordering::SeqCst);
        println!("parallel {}: {} usec, {} threads", run, elapsed, tcount);
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
