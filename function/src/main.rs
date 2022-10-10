fn main() {
    let _x = example_func(2 ,3);
    let _y = {
        let x = 3;
        x + 1
    };

    no_return();
    
    // diverge function
    fn forever() -> ! {
        loop {
          //...
        };
}


// prototype of a function
// keyword identifier(paramenters) -> return type {
// a function can be placed everywhere
fn example_func(i: i32, j: i32) -> i32
{
    let _x = 1; // statement don't return a value
    // return 5_i32 function can return early by return keyword.
    i + j // expression return a value
    // expression can't end with semicolon
}

fn no_return() -> () {
    println!("no return.")
}
