// & con trỏ tham chiếu đến biến
// * lấy trực tiếp giá trị của biến
fn main() {
	reference_and_borrow();
}

fn reference_and_borrow(){
	let mut vec: Vec<i32> = vec![1, 2, 3];
	let num: &mut i32 = &mut vec[2];
	let num2: &i32 = &*num; 

	// *num += 1;
	// println!("Third element is: {}", *num);
	// println!("Vector is: {:?}", vec);
	println!("{} {}", *num, *num2);

	let mut x = 1;
	let y = &x;
	println!("y: {y}");
	let z = *y;
	println!("z: {z}");
	x += z;
	println!("x: {x}");

	let mut vec_char: Vec<char> = vec!['h', 'e', 'l', 'l', 'o'];
	ascii_capitalize(&mut vec_char);
}

fn ascii_capitalize(v: &mut Vec<char>){
	let c = &v[0];

	if c.is_ascii_lowercase(){
		let up = c.to_ascii_uppercase();
		v[0] = up;
		println!("{:?}", v);
	}else{
		println!("Already capitalize: {:?}", v);
	}
}