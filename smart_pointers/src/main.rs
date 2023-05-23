pub mod reference_cycle {
    use std::{
        borrow::Borrow,
        cell::RefCell,
        rc::{Rc, Weak},
    };
    use List::{Cons, Nil};

    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    impl List {
        fn tail(&self) -> Option<&RefCell<Rc<List>>> {
            match self {
                Cons(_, item) => Some(item),
                Nil => None,
            }
        }
    }

    #[derive(Debug)]
    struct Node {
        value: i32,
        children: RefCell<Vec<Rc<Node>>>,
        parent: RefCell<Weak<Node>>,
    }

    pub fn main() {
        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("a initial Rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
        println!("a Rc count after b creation = {}", Rc::strong_count(&a));
        println!("b intital Rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b Rc count after changing a = {}", Rc::strong_count(&b));
        println!("a Rc count after changing a = {}", Rc::strong_count(&a));

        // // ERROR - STACK OVERFLOW
        // // println!("a next item = {:?}", a.tail());
        println!("------------------------------------------------------");

        let leaf = Rc::new(Node {
            value: 3,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        {
            let branch = Rc::new(Node {
                value: 5,
                children: RefCell::new(vec![Rc::clone(&leaf)]),
                parent: RefCell::new(Weak::new()),
            });
            *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

            println!(
                "branch strong = {}, weak = {}",
                Rc::strong_count(&branch),
                Rc::weak_count(&branch),
            );

            println!(
                "leaf strong = {}, weak = {}",
                Rc::strong_count(&leaf),
                Rc::weak_count(&leaf),
            );
        }

        println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf),
        );
        println!("------------------------------------------------------");
        let r1 = Rc::new(0);
        println!("r1 {} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));

        let r4 = {
            let r2 = Rc::clone(&r1);
            Rc::downgrade(&r2)
        };
        println!("r4 {} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));

        let r5 = Rc::clone(&r1);
        println!("r5 {} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));

        let r6 = r4.upgrade();
        println!("r6 {} {}", Rc::strong_count(&r1), Rc::weak_count(&r1));
    }
}

pub mod interior_mutatability {
    use std::{cell::RefCell, rc::Rc};

    pub trait Messenger {
        fn send(&self, msg: &str);
    }
    pub struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }
    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;
            let percentage_of_max = self.value as f64 / self.max as f64;
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You are used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You are used up over 75% of your quota");
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::RefCell;

        struct MockMessenger {
            send_messages: RefCell<Vec<String>>,
        }
        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    send_messages: RefCell::new(vec![]),
                }
            }
        }
        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.send_messages.borrow_mut().push(String::from(message));
                // ERROR at runtime - panic cause of multiple mutable borrowing
                // let mut one_borrow = self.send_messages.borrow_mut();
                // let mut two_borrow = self.send_messages.borrow_mut();

                // one_borrow.push(String::from(message));
                // two_borrow.push(String::from(message));
            }
        }

        #[test]
        fn it_send_an_over_75_percent_warning_message() {
            let mock_messenger = MockMessenger::new();
            let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);
            limit_tracker.set_value(80);
            assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
        }
    }

    #[derive(Debug)]
    enum List {
        Cons(Rc<RefCell<i32>>, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    pub fn main() {
        let value = Rc::new(RefCell::new(5));
        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;
        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}

pub mod ref_counted {
    use std::rc::Rc;
    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }

    use List::{Cons, Nil};
    pub fn main() {
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("Count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("Count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("Count after creating c = {}", Rc::strong_count(&a));
        }
        println!("Count after c goes out of scope = {}", Rc::strong_count(&a));
    }
}

pub mod sim_drop {
    use std::mem::drop;

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data: `{}`", self.data);
        }
    }

    pub fn main() {
        let c = CustomSmartPointer {
            data: String::from("my stuff"),
        };

        let d = CustomSmartPointer {
            data: String::from("other stuff"),
        };
        drop(c);
        println!("CustomSmartPointer created");
    }
}

pub mod sim_deref {
    use std::ops::Deref;

    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    pub fn main() {
        let x = 5;
        // let y = &x;
        // let y = Box::new(x);
        let y = MyBox::new(x);

        println!("{x}");
        println!("{}", *y);

        let name = MyBox::new(String::from("Rust"));
        hello(&name);
    }
}

pub mod use_box {
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};
    pub fn main() {
        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
}

fn main() {
    // use_box::main();
    // sim_deref::main();
    // sim_drop::main();
    // ref_counted::main();
    // interior_mutatability::main();
    reference_cycle::main();
}
