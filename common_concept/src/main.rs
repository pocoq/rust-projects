fn main() {
    // Print lyrics of "The twelve day" song
    the_twelve_day();

    // Generate the nth Fibonacci number
    let n = 9;
    let x = fib(n);
    println!("Fibonacci number of {n} is {x}");

	// Convert between F to C and reverse
	let f_to_c = convert_temperature(50.0, 'F');
	let c_to_f = convert_temperature(30.0, 'C');
	println!("Convert from F to C: {f_to_c}");
	println!("Convert from C to F: {c_to_f}");
}

fn the_twelve_day() {
    let days: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "fixth", "feventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let gifts: [&str; 12] = [
        "A partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut gift = String::new();
    let mut day;
    for i in 0..12 {
        day = days[i];

        gift = format!("{}\n{}", gifts[i], gift);

        println!("The {day} day of Christmas,");
        println!("My true love lent to me");
        println!("{gift}");
    }
}

fn fib(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn convert_temperature(d: f32, t: char) -> f32 {
    if t == 'F' {
        (d - 32.0) * 0.5556
    } else {
        (d * 1.8) + 32.0
    }
}
