#![allow(dead_code, unused_imports)]

use calculator::repl;
use calculator::characters::categorize_first_of;

fn main() {

    if let Some(val) = categorize_first_of("e") {
        println!("{:?}", val);
    }

    repl::run();
}
