mod statistics;

use rand::Rng; 
use self::statistics::stats_calculator_f64;

fn main() {
    let vect_size = get_user_input_as_usize();
    let rand_vect = generate_random_vector(vect_size);
    println!("");
    println!("Mean: {}", stats_calculator_f64::mean(&rand_vect));
    println!("Medium: {}", stats_calculator_f64::median(&rand_vect));
    println!("");
}


fn generate_random_vector(size: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();
    let mut vec = Vec::with_capacity(size);
    for _ in 0..size {
        vec.push(rng.gen());
    }
    return vec;
}

fn get_user_input() -> String {
    println!("Enter the size of the vector: ");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    return input;
}

fn get_user_input_as_usize() -> usize {
    let input = get_user_input();
    return input.trim().parse().expect("Failed to parse input");
}
