fn main() {
    // if statement
    // if condition1 == true {
    //     // A...
    // } else if condition2 == true {
    //     B...
    // }
    // else { C... }
    
    // for while loop
    for _ in 1..5 {
        println!("nn")
    }
    // for item in collection = for item in IntoIterator::into_iter(collection) | ownership of collection will be taken
    // for item in &collection = for item in collection.iter() | no move but item is immutable
    // for item in &mut collection = for item in collection.iter_mut() | no move and mutable
    
    // enumerate
    let a = [4, 3, 2, 1];
    // `.iter()` 方法把 `a` 数组变成一个迭代器
    for (i, v) in a.iter().enumerate() {
        println!("第{}个元素是{}", i + 1, v);
    }

    // continue break
    for i in 1..5 {
        if i == 2 {
            continue;
        }
        if i == 3 {
            break;
        }
        println!("{}", i);
    }

    // while condition == true {
        // do something
    // }

    // loop {
    //     need to be break
    //     will return a value
    //     break return value;
    // } 
}
