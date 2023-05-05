// Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) 
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn mean(numbers: &Vec<i32>) -> f32 {
    let sum: i32 = numbers.iter().sum();
    sum as f32 / numbers.len() as f32
}

fn median(numbers: &mut Vec<i32>) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]) as i32
    } else {
        numbers[mid]
    }
}

fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut map = HashMap::new();
    for i in numbers {
        let count = map.entry(i).or_insert(0);
        *count += 1;
    }

    let max_value = map.values().cloned().max().unwrap_or(0);

    map.into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(&k, _)| k)
        .collect()
}

pub fn calculate(v: &mut Vec<i32>) {
	println!("List integers: {:?}", v);

    let median = median(v);
    println!("Median: {:?}", median);

    let mode_v = mode(v);
    println!("Mode: {:?}", mode_v);
}
