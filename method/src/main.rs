//  classes in other languages | struct and enum in Rust
// class file {                | struct file { Data }
//    Data and methods.}       | impl File { Methods }
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // implementation
    fn new(w: u32, h: u32) -> Rectangle { // self is not show in parameters, this function is a correlation function
        Rectangle { width: w, height: h }
    }

    fn area(&self) -> u32 { // &self because of ownership
        self.width * self.height
    }

    fn width(&self) -> bool { // method has the same name with variable
        self.width > 0
    }
}

// enum and method
#![allow(unused)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // do something
    }
}

// method can be separated into multi places.
impl Rectangle{
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    let m = Message::Write(String::from("hello"));
    m.call();
}
