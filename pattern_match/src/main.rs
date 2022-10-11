fn main() {
    // match expression : every pattern should be included
    // match target {
    //     pattern1 => expression1,
    //     pattern2 => {
    //         statement1;
    //         statement2;
    //         expression2
    //     },
    //     _ => expression3       default case
    // }

    // if let  : only one pattern will be matched
    let v = Some(3u8);
    if let Some(3) = v {
        println!("three");
    }

    // match! macro : return true or false
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z'));

    // shading
    let age = Some(30);
    println!("before match, age={:?}",age);
    if let Some(age) = age {
        println!("when match, age={}",age);
    }
    println!("after match, age={:?}",age);

    // deconstruct Option
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    // categories of pattern
    //   * literal
    //   * deconstruct of array enum struct or tuple
    //   * variables
    //   * Wildcard
    //   * placeholder

    // while let
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    // pop all elements from stack
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // let PATTERN = EXPRESSION; let is actually a type of pattern matching as well.
    let (_x, _y, _z) = (1, 2, 3); // value will bound to the variable when successfully matched

    // function as a type of pattern matching.
    let point = (3, 5);
    print_coordinates(&point);

    // let for match : irrefutable, match must be successful.   |  if let : refutable 
    // let Some(x) = some_option_value;

    // literal
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // named variable
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // the outer scope y is shadowed
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {:?}", x, y);

    // .. and |
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3..=5 => println!("three through five"),
        _ => println!("anything"),
    }

    // deconstruct a struct
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    // let Point { x: a, y: b } = p;
    // assert_eq!(0, a);
    // assert_eq!(7, b);
    let Point {x, y} = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // deconstruct a enum
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}", r, g, b)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}", h, s, v)
        }
        _ => ()
    }

    // deconstruct a struct or tuple
    let ((_feet, _inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

    // _ will ignore some values
    // .. will ignore all the residual values

    // match guard
    let num = Some(4);

    match num {
        Some(x) if x < 5 => println!("less than five: {}", x), // if expression
        Some(x) => println!("{}", x),
        None => (),
    }

    // @ bound
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => { 
            // Captures any value that matches this range and binds that value to the variable id_variable
            println!("Found an id in range: {}", id_variable)
        },
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message2::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {  // x, y match the pattern i32, i32
    println!("Current location: ({}, {})", x, y);
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
 }
 
 enum Message {
     Quit,
     Move { x: i32, y: i32 },
     Write(String),
     ChangeColor(Color),
 }

 enum Message2 {
    Hello { id: i32 },
}