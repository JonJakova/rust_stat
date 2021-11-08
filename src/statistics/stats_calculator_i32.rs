pub fn mean(data: &[i32]) -> f64 {
    let mut sum = 0;
    for &v in data {
        sum += v;
    }
    sum as f64 / data.len() as f64
}

pub fn median(data: &[i32]) -> f64 {
    let mut data = data.to_vec();
    data.sort_unstable();
    if data.len() % 2 == 0 {
        (data[data.len() / 2] + data[data.len() / 2 - 1]) as f64 / 2.0
    } else {
        data[data.len() / 2] as f64
    }
}

pub fn mode(data: &[i32]) -> i32 {
    let mut counts: HashMap<i32, u32> = HashMap::new();
    for &v in data {
        *counts.entry(v).or_insert(0) += 1;
    }
    let mut max_count = 0;
    let mut max_value = 0;
    for (k, v) in counts {
        if v > max_count {
            max_count = v;
            max_value = k;
        }
    }
    max_value
}

pub fn variance(data: &[i32]) -> f64 {
    let mean = mean(data);
    let mut sum = 0.0;
    for &v in data {
        sum += (v - mean).powi(2);
    }
    sum / data.len() as f64
}

pub fn standard_deviation(data: &[i32]) -> f64 {
    variance(data).sqrt()
}

pub fn covariance(x: &[i32], y: &[i32]) -> f64 {
    let mean_x = mean(x);
    let mean_y = mean(y);
    let mut sum = 0.0;
    for (&x, &y) in x.iter().zip(y.iter()) {
        sum += (x - mean_x) * (y - mean_y);
    }
    sum / x.len() as f64
}

pub fn correlation(x: &[i32], y: &[i32]) -> f64 {
    covariance(x, y) / (standard_deviation(x) * standard_deviation(y))
}

pub fn skewness(data: &[i32]) -> f64 {
    let mean = mean(data);
    let mut sum = 0.0;
    for &v in data {
        sum += (v - mean).powi(3);
    }
    sum / data.len() as f64 / standard_deviation(data).powi(3)
}

pub fn z_score(x: i32, standard_deviation: f64) -> f64 {
    (x as f64 - mean(x)) / standard_deviation
}
