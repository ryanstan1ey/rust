#![allow(dead_code)]

// Enums usage
enum WebEvent {
 PageLoad,
 PageUnload,
 KeyPress(char),
 Paste(String),
 Click {x: i64, y: i64}
}


fn inspect(event: WebEvent) {
 match event {
  WebEvent::PageLoad => println!("page loaded"),
  WebEvent::PageUnload => println!("page unloaded"),
  WebEvent::KeyPress(c) => println!("pressed `{}`.", c),
  WebEvent::Paste(s) => println!("pressed \"{}\".", s),
  WebEvent::Click {x, y} => println!("clicked at x={} and y={}",x,y)
 }
}

// Type Aliases
enum VeryVerboseEnumOfThingsToDoWithNumbers {
 Add,
 Subtract
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

impl VeryVerboseEnumOfThingsToDoWithNumbers {
 fn run(&self, x: i32, y: i32) -> i32 {
  match self {
   Self::Add => x + y,
   Self::Subtract => x - y
  }
 }
}

enum Status {
 Rich,
 Poor 
}

enum Work {
 Civilian,
 Soldier
}

// enum with an implicit discriminator (starts at 0)
enum Number {
 Zero,
 One,
 Two
}

// enum with an explicit discriminator
enum Color {
 Red = 0xff0000,
 Green = 0x00ff00,
 Blue = 0x0000ff
}

fn main() {
 let pressed = WebEvent::KeyPress('x');
 let pasted = WebEvent::Paste("my text".to_owned());
 let click = WebEvent::Click {x: 20, y: 20};
 let load = WebEvent::PageLoad;
 let unload = WebEvent::PageUnload;

 inspect(pressed);
 inspect(pasted);
 inspect(click);
 inspect(load);
 inspect(unload);

 let x = Operations::Add;

 use crate::Status::{Rich, Poor};
 use crate::Work::*;

 let status = Poor;
 let work = Civilian;

 match status {
  Rich => println!("rich"),
  Poor => println!("poor")
 }

 match work {
  Civilian => println!("Civilian"),
  Soldier => println!("soldier")
 }

 println!("Zero is {}", Number::Zero as i32);
 println!("One is {}", Number::One as i32);

 println!("roses are #{:06x}", Color::Red as i32);
 println!("violets are #{:06x}", Color::Blue as i32);
}

