fn main() {
 let mut _mutable_integer = 7i32;
 {
  // When data is bound by the same name immutably, it also freezes.
  // Frozen data cannot be modified until the immutable binding goes out of scope.
  let _mutable_integer = _mutable_integer;

  // _mutable_integer = 50;
 }

 _mutable_integer = 10;
}