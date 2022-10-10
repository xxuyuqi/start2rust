#[allow(unused_variables)]
fn main() {
    // ownership
    let x = 5;
    let y = x;
    // x y are stored on the stack. The defination of y will allocate a new memory and copy the value of x
    println!("x :{}, y: {}", x, y);

    let s1 = String::from("hello");
    let s2 = s1; // move
    let s3 = s2.clone(); // deep copy
    takes_ownership(s3);
    let s4 = gives_ownership();
    let mut s5 = takes_and_gives_back(s2);
    // println!("s1: {}, s2: {}, s3: {}", s1, s2, s3);
    // s1 s2 are stored on the heap. The defination of s2 will take ownership of s1;

    // borrow
    let x = 5;
    let y = &x; // reference

    assert_eq!(5, x);
    assert_eq!(5, *y); // dereference

    calculate_length(&s5); // gives reference of s5, not the owership
    change(&mut s5);
    println!("{}", s5);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {

let some_string = String::from("hello");

some_string
}

fn takes_and_gives_back(a_string: String) -> String {

a_string
}

// Immutable references
fn calculate_length(s: &String) -> usize {
    s.len()
}

// mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// Only one mutable reference can exist at a time.
// Mutable and immutable references cannot exist at the same time.


// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }