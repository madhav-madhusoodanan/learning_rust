use std::env;

fn main() {
    cli();
    write();
    match_();
    read();
    hashes();
    ownership();
}

fn hashes() {
    use std::collections::HashMap;
    let mut marks = HashMap::new();

    marks.insert("hehe", 92);
    marks.insert("dude", 56); 

    println!("length: {}", marks.len());

    match marks.get("dude") {
        Some(marks) => println!("you got {}, dude!", marks),
        None => println!("F")
    }

    marks.remove("hehe");

    for (sub, mark) in &marks {
        println!("for {} you got {}", sub, mark);
    }

    println!("did you studi cpp? {}", marks.contains_key("cpp"));
}

fn read() {
    use std::io;
    let mut input = String::new();
    println!("Wassup!");

    match io::stdin().read_line(&mut input) {
        Ok(b) => println!("You said {}... nmber of bytes is {}", input.to_uppercase(), b),
        Err(e) => println!("damn, {}", e)

    }
}

fn match_() {
    let var = 12;
    match var {
        2 => println!("it is 2!"),
        3..=11 => println!("it is greater than 2!"),    // includes the last element also
        _ => println!("demnn")
    }
}

fn write() {
    use std::fs::File;
    use std::fs::OpenOptions;
    use std::io::prelude::*;

    let mut file = File::create("stuff.txt").expect("Fail");    // write only
    file = OpenOptions::new().write(true).read(true).open("stuff.txt").expect("Fail");  // multiple options

    file.write_all(b"yo!...").expect("Fail");
}

fn cli() {
    let args: Vec<String> = env::args().collect();

    for args in args.iter() {
        println!("{}", args);
            // 1st one is relative path to executable
            // then command line args
    }
}

fn ownership() {
    let mut x = String::from("yo ");
    x.push_str("hii");
    let mut y = x;
    y.push_str("haha");
    x = y;
    println!("{}", x);
}