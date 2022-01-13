#![allow(unused)]
// C style classic struct
#[derive(Debug)]
struct Person {
 name: String,
 age: u8,
}

// Unit struct - Field-less useful for Generics
struct Unit;

// Tuple struct - named tuples
struct Pair(i32, f32);

struct Point {
 x: f32,
 y: f32,
}


// Struct within a struct
#[allow(dead_code)]
struct Rectangle {
 top_left: Point,
 bottom_right: Point
}


fn main() {
 // creating a struct with field init shorthand
 let name = String::from("Jack Doe");
 let age = 27;
 let person = Person {name, age};
 println!("{:?}", person);

 // instantiating Points
 let top_left = Point {x: 1.0, y: 2.0}; 
 let bottom_right = Point {x: 3.0, y: 4.0}; 

 // accessing fields of struct
 println!("top_left x: {} and y: {}", top_left.x, top_left.y);
 println!("bottom_right x: {} and y: {}", bottom_right.x, bottom_right.y);


 // spreading
 let another_point = Point {x: 5.0, ..top_left};
 println!("result of spread: x::{} and y::{}", another_point.x, another_point.y);

 // destructuring a struct
 let Point {x: left_edge, y: top_edge} = top_left;

 let _rectangle = Rectangle {
  top_left: Point {x: left_edge, y: top_edge},
  bottom_right: bottom_right
 };

 let _unit = Unit;
 let pair = Pair(1, 0.1);

 println!("pair contains: {:?} and {:?}", pair.0, pair.1);

 // destructuring tuple struct 
 let Pair(integer, decimal) = pair;
 println!("pair contains {:?} and {:?}", integer, decimal);
}
