fn main() {
    let x = Some(10);
    let y = 10;
    match x {
        Some(50) => println!("Gotcha 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    match x {
        Some(50) => println!("Gotcha 50"),
        Some(n) if n == y => println! {"Matched, n = {n}"},
        _ => println! {"Default case, x = {:?}", x},
    }

    println!("------------------------------------");
    let x = 5;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let c = 't';
    match c {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7, z: 9 };
    let Point { x, y, z } = p;
    println!("{x} - {y}");

    println!("-------------------------------------");
    let msg = Message::ChangeColor(Color::Rgb(0, 255, 160));
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("{x} - {y}"),
        Message::Write(text) => println!("{text}"),
        Message::ChangeColor(Color::Rgb(r, g, b)) => println!("RGB: {r} - {g} -{b}"),
        Message::ChangeColor(Color::Hsv(h, s, v)) => println!("HSV: {h} - {s} -{v}"),
    }

    println!("-------------------------------------");
    let s = Some(String::from("Hello"));
    // if let Some(_s) = s {
    if let Some(_) = s {
        println!("Found a string");
    }
    println!("{:?}", s);

    let origin = Point { x: 0, y: 10, z: 8 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let number = (2, 3, 4, 5, 6, 7, 8);
    match number {
        (first, .., last) => println! {"{first} - {last}"},
    }

    println!("-------------------------------------");
    let msg = Message1::Hello { id: 15 };
    match msg {
        Message1::Hello {
            id: id_variable @ 3..=7,
        } => println!("found in range: {}", id_variable),
        Message1::Hello { id: 10..=12 } => {
            println!("found in other range: ")
        }
        Message1::Hello { id } => println!("Found some other id: {}", id),
    }
}
struct Point {
    x: i32,
    y: i32,
    z: i32,
}
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}
enum Message1 {
    Hello { id: i32 },
}
