use rand::{prelude::ThreadRng, thread_rng, Rng};
use std::time::Instant;

mod asin;

const EPSILON: f64 = 1e-10;
const CHECK_TRIES: u32 = 10_000;
const LOAD_TRIES: u64 = 10_000_000;

fn random_x(rng: &mut ThreadRng) -> f64 {
    // The expansion sum converges too long for an argument close (in modulo) to one.
    // So we'll cut you some corners.
    // random.random() * 1.8 - 0.9
    rng.gen_range(0.0..1.0) * 1.8 - 0.9
}

fn check_correctness(rng: &mut ThreadRng) -> bool {
    (0..CHECK_TRIES)
        .map(|_| {
            let x = random_x(rng);
            let a = x.asin();
            let b = asin::arcsine_as_sum(x, EPSILON);

            (x, a, b, (a - b).abs())
        })
        .all(|(x,a,b,r)| {
            if r < EPSILON {
                true
            } else {
                println!("asin({x}) = {a}, but calculated as {b}, diff is {r}", x = x, a = a, b = b, r = r);
                false
            }
        })
}

fn check_speed(rng: &mut ThreadRng) {
    println!("Generating test set...");

    println!("Testing...");

    let t_start = Instant::now();
    (0..LOAD_TRIES).map(|_| random_x(rng)).for_each(|x: f64| {
        asin::arcsine_as_sum(x, EPSILON);
        return ();
    });
    let t_end = Instant::now();
    let duration = t_end.duration_since(t_start);

    let cps = LOAD_TRIES as f64 / duration.as_secs_f64();
    println!("Calls per second: {}", cps);
    // println!("Processing of load took: {:?}", duration);
}

fn main() {
    let mut rng: ThreadRng = thread_rng();

    let ok = check_correctness(&mut rng);
    if ok {
        check_speed(&mut rng);
    }
}
