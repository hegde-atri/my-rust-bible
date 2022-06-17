pub fn run() {
    let mut original = String::from("original value");
    println!("\noriginal: \t\"{}\"", original);

    // changing scope for next variable
    {
        // let next = &mut original;
        // *next = String::from("next value");
        // println!("{}", next);
        // print_original(&original);
        change_original(&mut original);
        println!("Inner scope original: \t\"{}\"", original);
    }

    println!("{}", original);
}

fn print_original(original: &String){
    println!("fn print_original: \t\"{}\"", original);
}

fn change_original(original: &mut String){
    let next = original;
    *next = String::from("new original");
    println!("fn change_original: \t\"{}\"", next);
}

// specifying lifetime of variable
fn explicit_lifetime<'a>(p1: &'a i32, p2: &'a i32) -> &'a i32 {
    if p1 > p2 {
        p1
    } else {
        p2
    }
}

// Ownership and borrowing of functions and its parameters.
