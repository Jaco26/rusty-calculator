#![allow(dead_code, unused_imports)]

use std::error::Error;
use std::{io::{self, Write}};

mod terminal;



const LEFT_PAREN: &str = "(";
const RIGHT_PAREN: &str = ")";

struct Node {
    expr: String,
    children: Box<Vec<Node>>
}

impl Node {
    fn new() -> Node {
        Node {
            expr: String::new(),
            children: Box::new(vec![]),
        }
    }
}

struct OperationsTree {
    root: Option<Node>,
}

impl OperationsTree {
    fn new() -> OperationsTree {
        OperationsTree {
            root: None
        }
    }
    // fn parse(expr: &mut str) -> Result<(), Box<dyn Error>> {
    //     // node = Node::new();
    //     if false {
    //         return Err();
    //     }
    //     Ok(())

    // }
}

fn main() {
;
    let age = terminal::get_input("How old are you?").unwrap();
    let name = terminal::get_input("What's your name?").unwrap();
    println!("Hello! My name is {} and I am {} years old", name, age);
}
