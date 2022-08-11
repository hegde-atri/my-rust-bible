use std::{mem, io::stdin};

const ANOTHER_CONST: u8 = 42; // no fixed address.
static CONST_2:i32 = 123; // cannot be mutable, as it would be unsafe.
// To make it a static mut, you will need to enclose the code block with unsafe.

enum State {
    Locked,
    Failed,
    Unlocked
}

pub fn run(){
    // DATA TYPES
    {
    let a: u8 = 123; // u = unsigned
    println!("a = {}", a); // a is immutable
    let mut b: i8 = 0; // i = signed
    println!("b = {} before", b);
    b = 42;
    println!("b = {} after", b);

    let mut c = 123456789;
    println!("c = {}, takes up {} bytes", c, mem::size_of_val(&c));
    c = -1;
    println!("c = {}", c);

    let z: isize = 123;
    let size_of_z: usize = mem::size_of_val(&z);
    println!("z = {}, takes up {} bytes, {}-bit OS", z, size_of_z, size_of_z*8);

    let d: char = 'x';
    println!("{} is a char, size = {} bytes", d, mem::size_of_val(&d));

    // IEEE754 - how non whole numbers are represented.
    let e = 2.5;
    println!("{}, size = {} bytes", e, mem::size_of_val(&e));

    let g: bool = false;
    println!("{}, size = {} bytes", g, mem::size_of_val(&g));
    }

    // OPERATORS
    {
    let mut a = 2+3*4;
    a -= 2; // a = a-2;
    println!("remainder of {} / {} = {}", a, 3, (a%3));
    let a_cubed = i32::pow(a, 3);
    println!("{} cubed is {}", a, a_cubed);
    let b = 2.5;
    let b_cubed = f64::powi(b, 3);
    let b_to_pi = f64::powf(b, std::f64::consts::PI);
    println!("{} cubed = {}, {}^pi = {}", b, b_cubed, b, b_to_pi); // bitwise operators
    let c = 1 | 2; // 01 OR 10 == 11
    println!("1 | 2 == {}", c);
    // | OR, & AND, ^ XOR, ! NOR
    // >> and <<
    // for comparitive operators == < > and the or equal too variants
    }

    // SCOPE AND SHADOWING

    {
        scope_and_shadowing();
    }

    // Control Flow
    {
        let temp = 35;
        if temp > 30
        {
            println!("really hot outside!");
        }
        else if temp < 10
        {
            println!("really cold!");
        }else
        {
            println!("temp is okay!");
        }

        let day = if temp > 20 {"sunny"} else {"cloudy"};
        println!("today is {}", day);

        let mut x = 1;
        while x < 1000
        {
            x *= 2;
            if x == 64 {continue;} // now goes to the "next" loop
            println!("x = {}", x);
        }

        let mut y = 1;
        loop // while true
        {
            y *= 2;
            println!("y = {}", y);
            if y == 1<<10 {break;} // 2^10
        }

        for x in 1..11
        {
            println!("x = {}", x);
        }

        for (pos, y) in (30..41).enumerate()
        {
            println!("{}: {}", pos, y);
        }

        let country_code = 44;
        let country = match country_code {
            44 => "UK",
            46 => "Sweden",
            7 => "Russia",
            1..=1000 => "unknown",
            _ => "invalid"
        };
        println!("The country with code {} is {}", country_code, country);

        let x = false;
        let s = match x{
            true => "yes",
            false => "no"
        };
    }

    // Data Structures

    {
        struct Point{
            x: f64,
            y: f64
        }
        struct Line{
            start: Point,
            end: Point
        }

        fn structures(){
            let p = Point {
                x: 3.0,
                y: 4.0
            };
            println!("Point p is at ({}, {})", p.x, p.y);
        }
    }
}

pub fn combination_lock_game(){
    let code = String::from("1234");
    let mut state = State::Locked;
    let mut entry = String::new();

    loop {
        match state {
            State::Locked => {
                let mut input = String::new();
                match stdin().read_line(&mut input) {
                    Ok(_) => entry.push_str(&input.trim_end()),
                    Err(_) => continue
                }
                if entry == code {
                    state = State::Unlocked;
                    continue;
                }
                if !code.starts_with(&entry) {
                    state = State::Failed;
                }
            }
            State::Failed => {
                println!("FAILED");
                entry.clear();
                state = State::Locked;
                continue;
            }
            State::Unlocked => {
                println!("UNLOCKED");
                return;
            }
        }
     }
}

fn scope_and_shadowing()
{
    let a = 123;
    {
        println!("Inside-before: a = {}", a);
        let a = 5;
        println!("Inside-after: a = {}", a);
    }
    println!("Outside-before: a = {}", a);
}
