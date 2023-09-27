use rand::Rng;
use std::collections::HashMap;

fn main() {
    let mut int_vector = vec!();
    let random_range = rand::thread_rng().gen_range(10..50);
    for _ in 0..random_range {
        let random_number: i32 = rand::thread_rng().gen_range(1..=25);
        int_vector.push(random_number);
    }
    println!("{random_range}");

    println!("Before sorting: {:#?}", int_vector);

    int_vector.sort();

    println!("After sorting: {:#?}", int_vector);

    let result = get_median(&int_vector);
    println!("[x] The median is: {}, with vector length: {}", result.0, result.1);

    let mode = get_mode(&int_vector);

    println!("The mode is: {:?}", mode);
}


fn get_median(int_vector: &Vec<i32>) -> (i32, usize) {
    let len_vector = int_vector.len();
    let median: i32 = match len_vector % 2 {
        0 => {
            println!("Even");
            let middle1 = int_vector[len_vector / 2 - 1];
            let middle2 = int_vector[len_vector / 2];
            (middle1 + middle2) / 2
        },
        1 => {
            println!("Oven");
            int_vector[(len_vector -1) / 2]
        },
        _ => panic!(),
    };
    (median, len_vector)
}


fn get_mode (vector: &Vec<i32>) -> (&i32, i32) {
    let mut scores = HashMap::new();
    for value in vector {
        let counter = scores.entry(value).or_insert(0);
        * counter += 1;
    }
    println!("Counted occurrences: {:#?}", scores);
    let mut sorted_scores = scores.into_iter().collect::<Vec<_>>();
    sorted_scores.sort_by(|a, b| b.1.cmp(&a.1));
    sorted_scores[0]
}