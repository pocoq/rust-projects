pub mod generic {
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest<T>(list: &[T]) -> &T {
        let mut largest: &T = &list[0];
        for val in list {
            // if val > largest {
            //     largest = val
            // }
        }
        largest
    }

    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    impl Point<f32> {
        fn distance_from_origin(&self) -> f32 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    struct Point1<T, U> {
        x: T,
        y: U,
    }

    struct Point2<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point2<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point2<X2, Y2>) -> Point2<X1, Y2> {
            Point2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    enum Option<T> {
        Some(T),
        None,
    }

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }

    pub fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
        let char_list = vec!['n', 'm', 'a', 'q', 'o'];

        // let result = largest(&number_list);
        // println!("The largest number is {}", result);

        // let result = largest(&char_list);
        // println!("The largest char is {}", result);

        let p = Point { x: 1, y: 2 };
        let f: Point<f32> = Point { x: 2.3, y: 5.6 };
        println!("p.x: {}", p.x());
        println!("distance: {}", f.distance_from_origin());

        let mix = Point1 { x: 1, y: 5.6 };

        let p1 = Point2 { x: 5, y: 10.4 };
        let p2 = Point2 { x: "Hello", y: 't' };
        let p3 = p1.mixup(p2);
        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
}

pub mod traits {
    use ch10_generictrait::{NewsArticle, Summary, Tweet};

    pub fn main() {
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());

        // generic_traits::notify(&tweet);
    }
}

pub mod lifetime {
    // struct ImportantExcerpt<'a>{
    // 	part: &'a str,
    // }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    pub fn main() {
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }

        // let novel =  String::from("Call me Ishmael. Some years ago ... ");
        // let first_sentence = novel.split(".").next().expect("Could not find a '.'");
        // let i = ImportantExcerpt{
        // 	part: first_sentence
        // };
    }
}

pub mod combine {
    use std::fmt::Display;

    struct news {
        headline: String,
        content: String,
    }

    fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    pub fn main() {
        let x = String::from("Hello world");
        let y = String::from("We are the best");
        // let ann = news {
        //     headline: String::from("Weather in NYC"),
        //     content: String::from("Bad weather"),
        // };
        let ann = 5.6;
        let longest = longest_with_an_announcement(x.as_str(), y.as_str(), ann);
        println!("{:?}", longest);
    }
}

pub mod program {
    fn find_nth<T: Ord + Clone>(elems: &[T], n: usize) -> T {
        let mut elem_refs: Vec<&T> = elems.iter().collect();
        elem_refs.sort();
        let t = elem_refs[n];
        return t.clone();
    }

    struct TestResult {
        /// Student's scores on a test
        scores: Vec<usize>,

        /// A possible value to curve all scores
        curve: Option<usize>,
    }
    impl TestResult {
        pub fn get_curve(&self) -> &Option<usize> {
            &self.curve
        }

        /// If there is a curve, then increments all
        /// scores by the curve
        pub fn apply_curve(&mut self) {
            if let Some(curve) = self.curve {
                for score in self.scores.iter_mut() {
                    *score += curve;
                }
            }
        }
    }
}
fn main() {
    // generic::main();
    // traits::main();
    // lifetime::main();
    combine::main();
}
