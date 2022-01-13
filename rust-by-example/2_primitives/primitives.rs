#![allow(unused)]

fn main() {
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
}