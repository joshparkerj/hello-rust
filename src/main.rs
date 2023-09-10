use ferris_says::say;
use std::io::{stdout, BufWriter};

macro_rules! say_hello {
    () => {
        println!("Hello, macro world!")
    };
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
    println!("Hello, world!");
    say_hello!();
}
