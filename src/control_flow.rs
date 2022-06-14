pub fn run(){
    let word = "Dog";
    if word == "Duck" {
        println!("Quack");
    } else if word == "Dog" {
        println!("Woof");
    } else{
        println!("Not a duck!");
    }


    let animal = "Duck";
    match animal {
        "Duck" => println!("Quack"),
        "Dog" => println!("Bark"),
        _ => {}
    }

    let ndb_freq:u16 = 384;
    //switch statement / pattern matching.
    match ndb_freq {
        200..=500 => {
            println!("NDB frequency is valid");
        }
        // conditions
        ndb_freq if ndb_freq >= 200 && ndb_freq <= 500 => {
            println!("NDB frequency is valid");
        }
        _ => {
            println!("NDB frequency is not valid");
        }
    }

    // TODO: not sure i understand if let statements.
    if let animal = "Duck" {
        println!("Quack");
    }

    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 5 {
            continue;
        }
        println!("{}", counter);
        if counter == 10{
            break;
        }
    }

    counter = 1;
    while counter <= 10 {
        println!("{}", counter);
        counter += 1;
    }

    for index in 1..=10 {
        println!("{}", index);
    }

    let aircrafts = ["Boeing 737 MAX", "Airbus A320 Neo", "Boeing 777-300ER", "Boeing 777-200LR"];
    for aircraft in aircrafts.iter() {
        println!("{}", aircraft);
    }

}
