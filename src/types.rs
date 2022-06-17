use std::ops::{Add};
/*
Primitive types:
Integers: u8, i8, u16, i16, u32, i32, u64, i64, i128, u128 (i => signed u => unsigned)
(number corresponds to number of bits ==> of memory)
Floats: f32, f64
Boolean (bool)
Character (char)

Tuples
Arrays
*/


use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    //Find max size
    println!("Max i32: {}", i32::MAX);
    println!("Max i64: {}", i64::MAX);

    //Boolean
    let is_active:bool = true;
    //Get boolean from expression
    let is_greater:bool = 10 < 1;

    // Char
    let a1:char = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (is_active, is_greater, a1, face));


    let phrase = String::from("Test string");
    let letter = phrase.chars().nth(5);

    let value = match letter {
        Some(character) => character.to_string(),
        None => String::from("No Value")
    };
    println!("Value: {}", value);


    let mut flights:Vec<&str> = Vec::new();
    let vec_macro = vec![1, 2, 3, 4];
    // push pop remove insert
    flights.push("Item 1");
    flights.push("Item 2");
    flights.push("Item 3");
    flights.push("Item 4");
    flights.push("Item 5");

    for flight in flights.iter(){
        println!("{}", flight);
    }
    //risky
    let third = flights[2];
    //safe
    let fourth = flights.get(3);

    flights.insert(2, "Item 3");
    flights.remove(1);

    let mut flights_dec:VecDeque<&str> = VecDeque::new();
    flights_dec.push_front("Item 2");
    flights_dec.push_back("Item 3");
    flights_dec.push_front("Item 1");
    println!("There are {} items", flights_dec.len());

    // hash map and btree map. (btree better for sorting)
    // hashmap takes two generic types. no key collision checking
    let mut flights_hashmap = HashMap::new();
    flights_hashmap.insert("1", "Item 1");
    flights_hashmap.insert("2", "Item 2");
    flights_hashmap.insert("3", "Item 3");
    //check key collision
    let flight_number= "4";
    if !flights_hashmap.contains_key(flight_number){
        flights_hashmap.insert(flight_number, "Item 4");
    }

    // hash set and btree set
    let mut flights_hashset = HashSet::new();
    flights_hashset.insert(("1", "Item 1"));

    // Generics
    let vor = NavAid {
        name: String::from("DQN"),
        frequency: 114.5,
        data: String::from("VOR")};
    let ndb_data:Option<String> = Option::None;
    let ndb = NavAid {
        name: String::from("HKF"),
        frequency: 239,
        data: ndb_data};


}

struct NavAid<T, U> {
    name: String,
    frequency: T,
    data: U
}
//remember to add constraints when required to generic
fn add<T>(operand1: T, operand2: T) -> T
where T: Add<Output = T>
{
    operand1 + operand2
}