#![allow(unused)]
use std::mem;

fn reverse(pair: (i32, bool)) -> (bool, i32) {
 // `let` can be used to bind the members of a tuple to variables
 let (integer, boolean) = pair;

 (boolean, integer)
}

fn analyze_slice(slice: &[i32]) {
  println!("first element of the array is {}.", slice[0]);
  println!("second element of the array is {}.", slice[1]);
  println!("the slice has {} elements.", slice.len());
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
  //2.1 PRIMITIVE TYPES
  let logical: bool = true;
  
  let a_float: f64 = 1.0;
  
  let an_integer_prefix_notation: i32 = 5;
  let an_integer_suffix_notation = 5i32;

  let default_float = 3.0;
  let default_integer = 7;

  let mut inferred_type = 12; // The type i64 is inferred from the next line 
  inferred_type = 4294967296i64;

  let mut _mutable = 12;
  _mutable = 21;

  // error - cannot change type
  /* mutable = true; */

  // overwriting variables with shadowing
  let _mutable = true;


  //2.2 Literals and operators
  println!("1 + 2 = {}", 1u32 + 2);
  println!("1 - 2 = {}", 1i32 - 2);

  println!("{}", true && false);
  println!("{}", true || false);
  println!("{}", !false);
 
  // bitwise operations
  println!("0011 AND 1101 = {}", 0b0011 & 0b1101 );
  println!("0011 OR 1101 = {}", 0b0011 | 0b1101 );
  println!("0011 XOR 1101 = {}", 0b0011 ^ 0b1101 );
  println!("1 << 5 = {}", 1u32 << 5 );

  println!("improve readabilitiy: {}", 1_000_000u32);
 
  //2.3 Tuples
  let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);

  // extracting values from tuples using tuple indexing
  println!("long tuple first value {}", long_tuple.0);
  println!("long tuple tenth value {}", long_tuple.9);

  let tuple_of_tuples = ((1u8, 2u16, 3u32), (4u8, "hello"));
  
  println!("tuple of tuples: {:?}", tuple_of_tuples);
  
  // Note: too long tuples cannot be printed
  // let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
  // println!("too long tuple: {:?}", too_long_tuple);

  let pair = (1,true);
  println!("pair is {:?}", pair);
  println!("reversed pair is {:?}", reverse(pair));

  // notice trailing comma
  println!("one element tuple {:?}", (5u32,));

  println!("just integer despite brackets: {}", (5u32));

  // tuple destructuring
  let tuple = (1, "hello", 4.5, true);

  let (a, b, c, d) = tuple;
  println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

  let matrix = Matrix(1.2, 2.3, 3.4, 4.5);
  println!("{:?}", matrix);


  // 3. Arrays
  let xs: [i32; 5] = [1, 2, 3, 4, 5];

  // Initializing all values to a single value
  let ys: [i32; 500] = [0;500];

  println!("first element of xs: {} and second element: {}", xs[0], xs[1]);
  println!("length of ys: {}", ys.len());

  // Arrays are stack allocated
 println!("Array xs occupies {} bytes", mem::size_of_val(&xs));
 println!("Array ys occupies {} bytes", mem::size_of_val(&ys));

 // Arrays can be automatically borrowed as slices
 println!("borrow the whole array as a slice");
 analyze_slice(&xs);

 //out-of-bounds indexing causes compilation error
 // println!("{}", xs[10]);
}