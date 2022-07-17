use rand::{prelude::ThreadRng, thread_rng, Rng};
mod asin;

const CHECK_TRIES: u32 = 10_000;
const LOAD_TRIES: u32 = 10_000_000;

fn random_x(rng: ThreadRng) -> f64 {
    // The expansion sum converges too long for an argument close (in modulo) to one.
    // So we'll cut you some corners.
    // random.random() * 1.8 - 0.9
    rng.gen_range(0.0..1.0) * 1.8 - 0.9
}

fn check_correctness(rng: ThreadRng) -> bool {
    (0..CHECK_TRIES).for_each(|n| {
        let x = random_x(rng);
        let a = x.asin();
        let b = arcsine_as_sum(x, EPSILON)

    });
        if not math.isclose(a, b, abs_tol=EPSILON):
            print(f'asin({x}) = {a}, but calculated as {b}, diff is {a - b}')
            return False
    return True
}

fn check_speed(rng: ThreadRng) {
    println!("Generating test set...");
    data = [random_x() for _ in range(LOAD_TRIES)]

    print('Testing...')
    t_start = time.perf_counter()
    for x in data:
        _ = arcsine_as_sum(x, EPSILON)
    t_end = time.perf_counter()

    cps = LOAD_TRIES / (t_end - t_start)
    print('Calls per second:', cps)
}

fn main() {
    let epsilon: f64 = 1e-10;
    let mut rng: ThreadRng = thread_rng();


    println!("final result {} expected {}", asin::arcsine_as_sum(x, epsilon), x.asin());
}
