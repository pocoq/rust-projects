mod give_away_tshirt {
    use std::vec;

    #[derive(Debug, PartialEq, Copy, Clone)]
    pub enum ShirtColor {
        Red,
        Blue,
    }

    pub struct Inventory {
        pub shirts: Vec<ShirtColor>,
    }

    impl Inventory {
        fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
            user_preference.unwrap_or_else(|| self.most_stocked())
        }

        fn most_stocked(&self) -> ShirtColor {
            let num_red = 0;
            let num_blue = 0;

            for color in &self.shirts {
                match color {
                    ShirtColor::Red => num_red + 1,
                    ShirtColor::Blue => num_blue + 1,
                };
            }

            if num_red > num_blue {
                ShirtColor::Red
            } else {
                ShirtColor::Blue
            }
        }
    }

    pub fn main() {
        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Blue, ShirtColor::Red],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} get {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} get {:?}",
            user_pref2, giveaway2
        );
    }
}

pub mod closure {
    use std::thread;

    fn borrow_immutably() {
        println!("Immutable borrow");
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        let only_borrows = || println!("From closure: {:?}", list);
        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    fn borrow_mutably() {
        println!("\nMutable borrow");
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        let mut borrows_mutably = || list.push(7);
        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    fn take_ownership() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }

    /* Sort list of rectangle by width */
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    fn sort_rectangles() {
        let mut list = [
            Rectangle {
                width: 10,
                height: 1,
            },
            Rectangle {
                width: 3,
                height: 5,
            },
            Rectangle {
                width: 7,
                height: 12,
            },
        ];

        // list.sort_by_key(|r| r.width);

        // ERR - try to count number of times sort_by_key gets called by STRING
        // let mut sort_operations = vec![];
        // let value = String::from("by key called");
        // list.sort_by_key(|r| {
        // 	sort_operations.push(value);
        // 	r.width
        // });

        // Count number of times sort_by_key gets called by NUMBER
        let mut num = 0;
        list.sort_by_key(|r| {
            num += 1;
            r.width
        });

        println!("{:?}", list);
        println!("{}", num);
    }

    /* Clone a string */
    fn make_a_cloner<'a>(s_ref: &'a str) -> impl Fn() -> String + 'a {
        move || s_ref.to_string()
    }

    fn clone_string() {
        let s_own = String::from("Hello world");
        let cloner = make_a_cloner(&s_own);
        // drop(s_own);
        cloner();
    }

    pub fn main() {
        // Closure'll infer type of params, as below infer by first execution
        let example_closure = |x| x;
        let s = example_closure(String::from("hello"));
        // let n = match example_closure(5);

        // borrow_immutably();
        // borrow_mutably();
        // take_ownership();
        sort_rectangles();
    }
}

pub mod iterator {
    #[derive(Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    fn filter_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];
        let in_my_size = shoe_in_size(shoes, 10);
        println!("{:?}", in_my_size);
    }

    pub fn main() {
        let v = vec![1, 2, 3];
        let v_iter = v.iter();
        for val in v_iter {
            println!("Got: {}", val);
        }

        let v1: Vec<_> = v.iter().map(|x| x + 1).collect();
        // println!("{:?}", v1);
		
		filter_by_size();
    }
}

fn main() {
    // give_away_tshirt::main();
    // closure::main();
    iterator::main();
}
