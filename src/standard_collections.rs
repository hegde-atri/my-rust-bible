use std::collections::HashMap;

pub fn run(){
    hashmap();
}

fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("{:?}", a);
    let idx: usize = 0;
    println!("a[0] = {}", a[idx]); // idx must be usize
    // a.get returns an Option
    match a.get(6){
        Some(x) =>  println!("a[6] = {}", x),
        None => println!("No such element")
    }

    for x in &a { println!("{}", x) }
    a.push(33);
    a.push(44);
    let last_elem = a.pop(); // it returns an Option
    println!("last_elem = {:?}", last_elem);
    while let Some(x) = a.pop() {
        println!("{}", x);
    }
}

fn hashmap(){
    let mut shapes = HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);
    shapes.insert(String::from("pentagon"), 5);

    println!("a square has {} sides", shapes["square".into()]);

    for(key, value) in &shapes {
        println!("{}: {}", key, value);
    }
    shapes.insert("square".into(), 5);
    println!("{:?}", shapes);
    shapes.entry("circle".into()).or_insert(1);
    println!("{:?}", shapes);
    {
        let actual = shapes.entry("circle".into()).or_insert(2);
        *actual = 0;
    }
    println!("{:?}", shapes);
}
