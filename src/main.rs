use ferris_says::say;
use std::io::{stdout, BufWriter};

macro_rules! say_hello {
    () => {
        println!("Hello, macro world!")
    };
}

macro_rules! sentence {
    ($subject: literal, $verb: literal, $object: literal) => {
        println!("{} {} {}", $subject, $verb, $object);
    }
}

// This is the code from the getting started guide at https://www.rust-lang.org/learn/get-started
fn main() {
    let stdout = stdout();
    let message = String::from("(Steve Buscemi voice) Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();

    // println! is a macro that prints text to the console
    // macros look like functions, except their name ends with a bang
    // they get expanded into source code instead of generating function calls
    // note that Rust macros are expanded into abstract syntax trees
    // (Rust does not use string preprocessing for this)
    // macros are created using macro_rules!

    let n = 12349;
    let base10 = format!("{}", n);
    let base2 = format!("{:b}", n); // b for binary
    let base8 = format!("{:o}", n); // o for octal
    let base16 = format!("{:x}", n); // small x for hexadecimal with lower case letters
    let base16big = format!("{:X}", n); // big X for hexadecimal with upper case letters

    println!("Hello, world!");
    say_hello!();
    println!("I'm a Rustacean!");

    println!("Base 10: {}", base10);
    println!("Base 2:  {}", base2);
    println!("Base 8:  {}", base8);
    println!("Base 16: {}", base16);
    println!("Base 16: {}", base16big);
    sentence!("Doug", "is", "a really cool person");
}
