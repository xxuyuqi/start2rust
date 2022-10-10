#[allow(unused_variables)]
fn main() {
    // string (String &str OsString OsStr CsString CsStr), string can't be indexed
    let my_name: &str = "Pascal"; // slice of a proallocated string, my_name have three word stored in stack(String in heap, capacity, length)

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{}, {}", hello, world);

    let s1 = first_word(&s);
    println!("{}", s1);

    let raw_str = r"Escapes don't work here: \x3F \u{211D}";

    // methods of string

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tup.0;

    // struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // user1.email = String::from("anotheremail@example.com");
    // A struct instance must be declared mutable in order to modify its fields, 
    // and Rust does not support declaring a field of a struct as mutable

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // println!("{:?}", user1); user2 takes ownership of username in user1

    let f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
      };
   
      let f1_name = &f1.name;
      let f1_length = &f1.data.len();
   
    //   println!("{:?}", f1);
      println!("{} is {} bytes long", f1_name, f1_length);
    // When you transfer ownership of a field out of the struct,
    // you can no longer access that field, but you can access all other fields normally.
    //                Data in memory
    //                            File struct
    //    Field name:             name                data
    //Field data type:            string              Vec<u8>
    //In-memory representation:  ptr|size|capacity    ptr|size|capacity
    //                 In heap:  [u8; name.size]      [u8; data.size]

      // tuple struct
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // unit-like struct
    struct AlwaysEqual;
    let subject = AlwaysEqual;

    // enum
    enum PokerSuit {
        Clubs,
        Spades,
        Diamonds,
        Hearts,
      }
      let heart = PokerSuit::Hearts;

      // Option enum
    //   enum Option<T> {
    //     Some(T),
    //     None,
    // }
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None); // use match to process Option

    // array Vector
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // stored in stack
    let b = [3; 5];
    let first = a[0]; // [T] type
    let slice: &[i32] = &a[1..3]; // &[T] type
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    &s[..1]
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[allow(dead_code)]
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

#[derive(Debug)] // to print struct
 struct File {
   name: String,
   data: Vec<u8>,
 }

 // dbg! > stderr println! > stdout
 fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}