fn main() {
    /* 
        multi line comments
    */
    basic();
    function(10);
    reference();
    structs();
    scoped_function();
    strings();
    implementation();
    vector();
    files();

}

fn basic() {
    const MAX:i32 = 1;  // type definition OP
    enum Something {
        One,
        Two,
        Three,
    }
    let arr = (1, 2.6, "hehe");
    let (a, b,c) = arr;
    let player:Something = Something::One;


    // more like a switch-case
    match player {
        Something::One => {println!("nice way to return: {}", add(MAX, a))},
        Something::Two => {println!("{}", b)},
        _ => {println!("{}", c)},
    }
}

fn function(num: i32) -> i32 {
    
    println!("{}", num);
    return num;
}

fn add(a: i32, b: i32) -> i32 {
    // dope
    // to return (another way) , remove the semicolons
    a + b
}

fn reference() {
    let mut a = 1;
    let b = &mut a;

    *b = 2; // yk, references need that asterisk to do something
    println!("mutable references {}", a);

    let function = |&x:&i32| {return x + 1;};   // thats a closure
    println!("closure returns {}", function(&10));
}

fn structs() {
    struct Color {
        red: u8,
        green: u8,
        blue: u8,
    }

    let mut bg = Color{red: 0xFA, green: 0xFF, blue: 0xFF};

    bg.blue = 0x00;
    println!("Colors: {} {} {}", bg.red, bg.green, bg.blue);

    struct AltColor(u8, u8, u8);

    let bg_1 = AltColor(255, 50, 10);
    println!("AltColors: {} {} {}", bg_1.0, bg_1.1, bg_1.2);
}

fn scoped_function() {
    let a = 1;
    fn mock(a: i32){   
        let b = 2;
        println!("in mock, a:{}", a);
        println!("in mock, b:{}", b);
    }
    println!("in scoped_function, a:{}", a);
    mock(a);
    mock(a);
}

fn strings() {
    let str = String::from("hello");
    let simple_str = "hello wut are you doin?";
    // dk why it is kinda weirr...

    println!("simple_str: {}", simple_str.len());
    println!("str: {}", str.len());
    println!("is empty? {}", simple_str.is_empty());
    
    for token in simple_str.split_whitespace() {
        println!("{}", token);
    }

    println!("find 'wut' -> {}" , simple_str.contains("wut"));
    // this guy is searching the whole string just like that. WOW!

}

fn implementation() {
    struct Rect{
        width: u32,
        height: u32
    }
    impl Rect{
        fn describe(&self) {
            println!("{} x {}", self.width, self.height);
        }
    }

    let rect = Rect{width: 10, height: 5};
    rect.describe();
}

fn vector() {
    let vector:Vec<i32> = Vec::new();
    let mut vectr = vec![1, 2, 3, 4];
    println!("{}", vectr[2]);

    vectr.push(49);
    vectr.remove(2); // removes index 2

    for number in vectr.iter() {
        print!("{} ", number);
    }
}

fn files(){
    use std::fs::File;
    use std::io::prelude::*;

    let mut file = File::open("stuff.txt").expect("Failed to open");    // read only
    let mut contents = String::new();   // empty string

    file.read_to_string(&mut contents).expect("Fffffffffff");  // gotta pass a mutable ref to contents
    println!("{}", contents);
}
