// use crate::exercises::exercise_1;
use crate::exercises::{exercise_1, exercise_2};
use std::collections::HashMap;

pub mod exercises;

fn main() {
    // store_in_vector();

    // utf8_string();

    // hash_map();

    // let mut v = vec![
    //     36, 43, 1, 1, 2, 2, 2, 2, 3, 26, 34, 36, 36, 38, 36, 43, 43, 43, 54, 76, 328, 378, 2928,
    // ];
    // exercise_1::calculate(&mut v);

    let sentence = "octopus is comming";
    let pigfy_sentence = exercise_2::pigify(&sentence);
	println!("{pigfy_sentence}");
}

fn store_in_vector() {
    let mut x: Vec<i32> = Vec::new();

    x.push(1);
    println!("list: {:?}", x);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let second: Option<&i32> = v.get(1);
    match second {
        Some(second) => println!("{second}"),
        None => println!("No element"),
    }

    // let does_not_exist = &v[100];
    let does_not_exist = v.get(100);
    println!("{:?}", does_not_exist);

    let mut v = vec![100, 45, 78, 98, 46];
    for n_ref in &v {
        let n_plus_one: i32 = *n_ref + 1;
        println!("{n_plus_one}");
    }

    for n_ref in &mut v {
        *n_ref += 50;
    }
    println!("{:?}", v);

    // hold different type
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(2.5),
        SpreadsheetCell::Text(String::from("Blue")),
    ];
}

fn utf8_string() {
	let mut a = String::from("hello");
	let b = " world".to_string();
	let c = a + &b;
	println!("c: {c}  -- b:{b}");

	// a.push_str(" world !!");
	// println!("a: {a}");

    let hello = "Здравствуйте";
    for c in hello.chars() {
        println!("{}", c);
    }

    for b in hello.bytes() {
        println!("{}", b);
    }
}

fn hash_map() {
    //General
    println!("-- General --");
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Green"), 15);

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Point of Blue team: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Ownership
    println!("-- Ownership --");
    let mut fav_field = String::from("Red");
    let mut fav_value = String::from("100");

    let mut map = HashMap::new();
    map.insert(&fav_field, &fav_value);
    println!("Variable: {} - {}", fav_field, fav_value); // Error cause of value moving from variables to map

    for (key, value) in &map {
        println!("{key}: {value}");
    }

    // Overwrite
    println!("-- Overwrite --");
    scores.insert(String::from("Blue"), 1000);
    println!("{:?}", scores);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map_text = HashMap::new();

    for word in text.split_whitespace() {
        let count = map_text.entry(word).or_insert(0);
        *count += 1
    }
    println!("{:?}", map_text);

    let mut h: HashMap<char, Vec<usize>> = HashMap::new();
    for (i, c) in "hello!".chars().enumerate() {
        h.entry(c).or_insert(Vec::new()).push(i);
    }

    let mut sum = 0;
    for i in h.get(&'l').unwrap() {
        sum += *i;
    }
    println!("{}", sum);
}
