fn main() {
    // Vector
    let mut v = Vec::new();
    v.push(1);

    let _v2: Vec<i32> = Vec::with_capacity(10); // create a vector with capacity of 10.
    let v3 = vec![1, 2, 3];
    
    // 2 method to read a value from vector
    let value = &v3[1];
    println!("{:}", *value);

    match v.get(3) {
        Some(c) => println!("{}", c),
        None => println!("no number"),        
    }

    if let Some(va) = v3.get(2){
        println!("if let : {}", va)
    };

    // iter
    for i in &mut v {
        *i += 10;
    }

    // store different type
    // with enum
    let v4 = vec![
        IpAddr::V4("127.0.0.1".to_string()),
        IpAddr::V6("::1".to_string())
    ];
    for ip in v4 {
        show_addr(ip)
    }

    // with feature object
    let v5: Vec<Box<dyn IpAddr2>> = vec![
        Box::new(IV4("127.0.0.1".to_string())),
        Box::new(IV6("::1".to_string())),
    ];

    for ip in v5 {
        ip.display();
    }

    // HashMap
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, 1.0);

    let _map2: HashMap<i32, i32> = HashMap::with_capacity(10);

    // Vector 2 HashMap
    let teams_list = vec![
        ("CN".to_string(), 100),
        ("USA".to_string(), 10),
        ("JP".to_string(), 50),
    ];

    let teams_map: HashMap<_,_> = teams_list.into_iter().collect();
    println!("{:?}", teams_map);

    // get return Option<T>

    // iter
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Hash Funtion can be changed 
    // let mut hash: HashMap<_, _, BuildHasherDefault<XxHash64>> = Default::default();

}

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String)
}

fn show_addr(ip: IpAddr) {
    println!("{:?}",ip);
}

trait IpAddr2 {
    fn display(&self);
}

struct IV4(String);
impl IpAddr2 for IV4 {
    fn display(&self) {
        println!("ipv4: {:?}",self.0)
    }
}
struct IV6(String);
impl IpAddr2 for IV6 {
    fn display(&self) {
        println!("ipv6: {:?}",self.0)
    }
}
