pub fn run(){
    pattern_matching();
}

enum Colour{
    Red,
    Green,
    Blue,
    RgbColour(u8,u8,u8), // tupe
    Cmyk{cyan: u8, magenta: u8, yellow: u8, black: u8}, //struct
}

fn enums(){
    let c:Colour = Colour::Red;
    match c {
        Colour::Red => println!("R"),
        Colour::Green => println!("G"),
        Colour::Blue => println!("B"),
        //Colour::RgbColour(0,0,0) | Colour::Cmyk { cyan: _, magenta: _, yellow: _, black: 255 }  => println!("black"),
        Colour::RgbColour(0,0,0) | Colour::Cmyk {black: 255, ..}  => println!("black"),
        // What we can do is replace the {} with {black:255,..} to say ignore the rest!
        Colour::RgbColour(r,g,b) => println!("rgb({},{},{})", r, g, b),
        Colour::Cmyk { cyan: _, magenta: _, yellow: _, black: _ } => println!("")

    }
}

fn unions_lesson(){
    union IntOrFloat{
        i: i32,
        f: f32
    }

    let mut iof = IntOrFloat { i: 123 };
    iof.i = 234;

    let value = unsafe { iof.i };

    fn process_value(iof: IntOrFloat){
        unsafe{
            match iof{
                IntOrFloat {i:42} => println!("meaning of life value"),
                IntOrFloat {f} => println!("value = {}", f)
            }
        }
    }
}

fn option_lesson(){
    let x = 3.0;
    let y = 0.0;

    let result = if y != 0.0 { Some(x/y)} else {None};
    // Option => some(t) | None
    match result {
        Some(z) => println!("result {}", z),
        None => println!("Cannot divide by 0")
    }
    if let Some(z) = result {println!("result is {}", z)}
}

fn array(){
    let mut a: [i32;5] = [1, 2, 3, 4, 5];
    println!("a has {} elements, first is {}", a.len(), a[0]);
    a[2] = 320;
    println!("a[2] = {}", a[2]);
    // print out entire array
    println!("{:?}", a);
    // cannot compare arrays with diff size
    if a != [1, 2, 3, 4, 5] {
        println!("does not match");
    }

    let b = [1u16; 10]; //fills it with 10 '1's
    for i in 0..b.len(){
        println!("{}", b[i]);
    }
    println!("b took up {} bytes", std::mem::size_of_val(&b));

    let mtx:[[f32;3]; 2] = [
        [1.0,0.0,1.0],
        [0.0,2.0,3.0]
    ];
    println!("{:?}", mtx);
    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i == j {
                println!("mtx[{}][{}] = {}", i, j, mtx[i][j]);
            }
        }
    }
    let mut data = [1,2,3,4,5];

    fn use_slice(slice: &mut [i32]){
        println!("1 = {}, length = {}", slice[0], slice.len());
    }

    use_slice(&mut data[1..4]);
    use_slice(&mut data);
}

fn tuples(){
    let x = 3;
    let y = 5;

    fn sum_and_product(x: i32, y: i32) -> (i32, i32){
        (x+y, x*y)
    }

    let sp = sum_and_product(x, y);

    println!("sp = {:?}", sp);
    println!("sum = {0}, product = {1}", sp.0, sp.1);
    // destructuring
    let (a, b) = sp;
    let sp2 = sum_and_product(3, 7);
    let combined = (sp, sp2);
    println!("{:?}", combined);
    println!("{}", (combined.1).1);
    let meaning = (42,);

}

fn pattern_matching(){

    fn how_many(x: i32) -> & 'static str {
        match x{
            0 => "no",
            1 | 2 => "one or two",
            12 => "a dozen",
            z @ 9..=11 => "Lots of oranges",
            _ if (x % 2 == 0) => "some",
            _ => "a few"
        }
    }

    for x in 0..13{
        println!("{}: I have {} oranges", x, how_many(x));
    }

    let mut point = (3,4);
    match point {
        (0,0) => println!("origin"),
        (0,ref mut y) => println!("y"),
        (_,0) => println!("x"),
        (x,y) => println!("x,y")
    }
}

fn generics(){
    struct Point<T>{
        x: T,
        y: V
    }
    struct Line<T>{
        start: Point<T>,
        end: Point<T>
    }

    let a:Point<f64> = Point{ x: 0.0, y: 1.0};
    let b:Point<f64> = Point{ x: 1.2, y: 2.3 };
    let myline = Line {start: a, end: b};
}
