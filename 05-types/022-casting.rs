// #![allow(overflowing_literals)]

fn main() {
  let decimal = 65.4321_f32;

  // error: no implicit casting
  // let integer: u8 = decimal;

  let integer = decimal as u8;
  let character = integer as char;
  println!("Casting: {} -> {} -> {}", decimal, integer, character);

  // when casting any value to an unsigned type, T,
  // std::T::MAX + 1 is added or subtracted until the value
  // fits into the new type

  println!("1000 as u16: {}", 1000 as u16);
  // 1000 - 256 - 256 - 256 = 232
  println!("1000 as u8: {}", 1000 as u8);
  // -1 + 256 = 255
  println!("-1 as u8: {}", (-1i8) as u8);
  println!("1000 mod 256 is: {}", 1000 % 256);

  // When casting to a signed type, the (bitwise) result is the same as
  // first casting to the corresponding unsigned type. If the most significant
  // bit of that value is 1, then the value is negative.

  println!("128 as i16: {}", 128 as i16); // 128
  println!("128 as i8: {}", 128 as i8); // -128
  // 232 - 256 = -24 ???
  println!("232 as i8: {}", 232 as i8); // -24
  println!("233 as i8: {}", 233 as i8);
  println!("300 as i8: {}", 300 as i8);
}