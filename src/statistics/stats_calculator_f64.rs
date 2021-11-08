// use std::collections::HashMap;

pub fn mean(values: &[f64]) -> f64 {
    let mut sum = 0.0;
    for value in values {
        sum += *value;
    }
    sum / values.len() as f64
}

pub fn median(values: &[f64]) -> f64 {
    let mut values = values.to_vec();
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if values.len() % 2 == 0 {
        (values[values.len() / 2 - 1] + values[values.len() / 2]) / 2.0
    } else {
        values[values.len() / 2]
    }
}


// Mode cannot be used for floating point numbers
// due to rust not supporting f64 comparison in HashMap
// Consider using a struct which implements `Eq`

// pub fn mode(values: &[f64]) -> f64 {
//     let mut counts = HashMap::new();
//     for value in values {
//         let count = counts.entry(*value).or_insert(0);
//         *count += 1;
//     }
//     let mut max_count = 0;
//     let mut max_value = 0.0;
//     for (value, count) in counts {
//         if count > max_count {
//             max_count = count;
//             max_value = value;
//         }
//     }
//     max_value
// }

pub fn variance(values: &[f64]) -> f64 {
    let mean = mean(values);
    let mut sum = 0.0;
    for value in values {
        sum += (value - mean).powi(2);
    }
    sum / values.len() as f64
}

pub fn standard_deviation(values: &[f64]) -> f64 {
    variance(values).sqrt()
}

pub fn covariance(x: &[f64], y: &[f64]) -> f64 {
    let mean_x = mean(x);
    let mean_y = mean(y);
    let mut sum = 0.0;
    for i in 0..x.len() {
        sum += (x[i] - mean_x) * (y[i] - mean_y);
    }
    sum / x.len() as f64
}

pub fn correlation(x: &[f64], y: &[f64]) -> f64 {
    covariance(x, y) / (standard_deviation(x) * standard_deviation(y))
}

pub fn z_score(x: f64, mean: f64, standard_deviation: f64) -> f64 {
    (x - mean) / standard_deviation
}

pub fn z_score_from_data(x: f64, data: &[f64]) -> f64 {
    z_score(x, mean(data), standard_deviation(data))
}

pub fn z_score_from_data_with_mean(x: f64, data: &[f64], mean: f64) -> f64 {
    z_score(x, mean, standard_deviation(data))
}

pub fn z_score_from_data_with_mean_and_standard_deviation(
    x: f64,
    data: &[f64],
    mean: f64,
    standard_deviation: f64,
) -> f64 {
    z_score(x, mean, standard_deviation)
}
