pub fn strlen(s: impl AsRef<str>) -> usize {
    s.as_ref().len()
}

pub fn strlen2<S>(s: S) -> usize
where
    S: AsRef<str>,
{
    s.as_ref().len()
}

pub fn foo() {
    strlen("hello"); // &'static str
    strlen(String::from("hello")); // String
}

// Monomorphization

pub fn strlen_refstr(s: &str) -> usize {
    s.len()
}

pub fn strlen_string(s: String) -> usize {
    s.len()
}

// =========================================================

pub trait Hello {
    fn hello(&self);
}

impl Hello for &str {
    fn hello(&self) {
        println!("hello {}", self);
    }
}

pub fn foo2() {
    "J".hello();
}


// Static Dispatch

pub fn bar(h: impl Hello) {
    h.hello();
}

// pub fn bar(h: &str) {
//     h.hello();
// }

// =========================================================

// Trait Objects

// pub fn foo3() {
//     for elem in vec!["Hi", "Hello!"] {
//         elem.hello();
//     }
// }

pub fn bar2(s: &[impl Hello]) {
    for elem in s {
        elem.hello();
    }
}

pub fn foo4() {
    bar2(&["Hi", "Hello!"]); 
    // bar2(&[String::from("Hi"), "Hello!"]); 
}



// Dyn

pub fn bar3(s: &[&dyn Hello]) {
    for elem in s {
        elem.hello();
    }
}

// =========================================================

// Sized trait
// https://doc.rust-lang.org/std/marker/trait.Sized.html

// Need size for args
// pub fn strlen<S: AsRef<str>>(s: S) -> usize {
//     s.as_ref().len()
// }



// =========================================================

// Sizing Unsized Types

// &
// Box (https://doc.rust-lang.org/std/boxed/struct.Box.html)
// Rc

// pub fn bar5(s: Box<dyn Hello>) {
//     s.hello();
// }


// =========================================================

// Dynamic Dispatch

pub fn say_hello(s: &dyn Hello) {
    s.hello(); // how to call? 
}

// https://doc.rust-lang.org/reference/dynamically-sized-types.html

// &dyn Hello
// stored in &
// 1. pointer to actual, concrete, implementing type
// 2. pointer to a vtable for the referenced trait

// vtable?



// =========================================================

// Limitation: Multiple Traits

// pub fn baz(s: &dyn Hello) {
    // s.hello();
    // let s = s.as_ref();
    // s.len();

    // s::weird();
// }

// Solution?

// https://doc.rust-lang.org/std/marker/trait.Send.html



// =========================================================

// Limitation: Associated Types
// Limitation: Static Trait Methods

pub trait Hello2 {
    // type Name;
    fn weird(&self) {}

    fn weird_static() where Self: Sized {}

    fn hello2(&self);
}

impl Hello2 for &str {
    // type Name = ();
    fn weird(&self) {
        Self::weird_static();
    }

    fn weird_static() where Self: Sized {
        println!("FOR STR!");
    }

    fn hello2(&self) {
        println!("hello {}", self);
    }
}

// pub fn say_hello2(s: &dyn Hello2) {
//     s.hello2();
// }

// solution for Methods


// Limitation: Generic Methods

// https://doc.rust-lang.org/std/iter/trait.Extend.html

// use std::iter::Extend;
// struct MyVec<T>(Vec<T>);

// impl<T> Extend<T> for MyVec<T> {
//     fn extend<I>(&mut self, iter: I)
//     where 
//         I: IntoIterator<Item = T>
//     {
//         // ...
//     }
// }


// Limitation: No Non-Receiver Self

// fn clone(&self) -> Self
// pub fn clone(v: &dyn Clone) {
//     let x = v.clone(); // what size is x?
// }


// Partial Object Safety
// https://doc.rust-lang.org/std/iter/trait.Iterator.html


// Object safety
// https://doc.rust-lang.org/reference/items/traits.html#object-safety
