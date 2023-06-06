pub mod unsafe_rust {
    use std::slice;

    fn split_at_mut_1(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);
        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    static HELLO_WORLD: &str = "Hello, world";
    unsafe fn dangerous() {}

    extern "C" {
        fn abs(input: i32) -> i32;
    }

    #[no_mangle]
    pub extern "C" fn call_from_C() {
        println!("Just call a Rust function from C!");
    }

    pub fn main() {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        let address = 0x012345usize;
        let r = address as *const i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }

        unsafe {
            dangerous();
        }

        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];

        let (a, b) = split_at_mut_1(r, 3);
        // let (a, b) = r.split_at_mut(3);
        println!("{:?} - {:?}", a, b);

        unsafe {
            println!("Absolute value of -3 accoring to C:{}", abs(-3));
        }

        println!("name is: {}", HELLO_WORLD);

        let mut v = Vec::with_capacity(4);
        for i in 0..3 {
            v.push(i);
        }
        println!("{:?}", v);
    }
}

pub mod advanced_trait {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    struct Milimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Milimeters {
        type Output = Milimeters;
        fn add(self, other: Meters) -> Milimeters {
            Milimeters(self.0 + (other.0 * 1000))
        }
    }

    /*********************************************** */
    trait Animal {
        fn baby_names() -> String;
    }

    struct Dog;
    impl Dog {
        fn baby_names() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_names() -> String {
            String::from("puppy")
        }
    }
    /*************************************************** */
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arm furiously*");
        }
    }
    /************************************************ */
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }

    impl OutlinePrint for Point {}
    /************************************************* */
    // use std::fmt;
    struct Wrapper(Vec<String>);

    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    pub fn main() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
        let p1 = Point { x: 3, y: 7 };
        p1.outline_print();
        /********************************************* */
        println!("A baby dog is called a {}", <Dog as Animal>::baby_names());

        /********************************************* */
        let person = Human;
        Pilot::fly(&person);
        Wizard::fly(&person);
        person.fly();
        /********************************************* */
        let w = Wrapper(vec![String::from("hello"), String::from("world")]);
        println!("w = {}", w);
    }
}

pub mod advanced_type {
    use std::fmt;
    use std::io::Error;
    /* Alias type */
    type Result<T> = std::result::Result<T, Error>;
    pub trait Write {
        fn write(&mut self, buf: &[u8]) -> Result<usize>;
        fn flush(&mut self) -> Result<()>;
        fn write_all(&mut self, buf: &[u8]) -> Result<()>;
        fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<()>;
    }

    /* The never type */

    /* Unsized type */

    fn is_equal<T: Eq>(t1: T, t2: T) -> bool {
        t1 == t2
    }

    pub fn main() {
        type Kilometers = i32;
        let x: i32 = 5;
        let y: Kilometers = 10;
        println!("x + y = {}", x + y);

        println!("{}", is_equal(String::from("Hello"), String::from("world")));
    }
}

pub mod advanced_func {
    fn add_one(x: i32) -> i32 {
        x + 1
    }

    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    enum Status {
        Value(u32),
        Stop,
    }

    fn return_closure() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
	
    pub fn main() {
        let answer = do_twice(add_one, 5);
        println!("{answer}");
        /****************************************** */
        let list_of_number = vec![1, 2, 3];
        let list_of_string: Vec<String> =
            list_of_number.iter().map(|i| (i + 1).to_string()).collect();
        println!("{:?}", list_of_string);
        /****************************************** */
        let list_of_status: Vec<Status> = (0u32..20).map(Status::Value).collect();
    }
}

pub mod macros{
	pub fn main(){
		
	}
}

fn main() {
    // unsafe_rust::main();
    // advanced_trait::main();
    // advanced_type::main();
    // advanced_func::main();
	macros::main();
}
