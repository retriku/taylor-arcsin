fn nn_fact_div_n_fact_squared(n: i32) -> f64 {
    if n == 0 {
        0.0
    } else {
        (1..n).fold(2.0, |acc, i| acc * (((n + i) as f64) / i as f64))
    }
}

pub fn arcsine_as_sum(x: f64, epsilon: f64) -> f64 {
    let mut delta: f64 = 1.0;
    let mut n: i32 = 1;
    let mut arcsin: f64 = x;
    let mut n4: f64 = 4.0;
    let mut xn: f64 = x;

    while delta.abs() >= epsilon / 5.0 {
        let d: f64 = nn_fact_div_n_fact_squared(n);
        let c: f64 = 2.0 * n as f64 + 1.0;
        let x2np1: f64 = x * xn * xn;

        let arcsin1 = arcsin + (d * x2np1) / (n4 * c);

        delta = arcsin1 - arcsin;

        arcsin = arcsin1;

        n += 1;
        n4 *= 4.0;
        xn *= x;
    }

    return arcsin;
}
