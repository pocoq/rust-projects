#[derive(Debug)]
enum UsState{
	Texas,
	California
}

enum Coin{
	Niken,
	Quarter(UsState),
	Penny,
	Dime
}

fn main() {
    let config_max = Some(10u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

	let mut count = 0;
	let coin = Coin::Penny;
	if let Coin::Quarter(state) = coin{
		println!("{:?}", state);
	}
	else{
		count += 1;
	}

	println!("{count}");
}
