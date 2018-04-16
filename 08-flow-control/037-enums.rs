#[allow(dead_code)]
#[derive(Debug)]
enum Color {
  Red,
  Blue,
  Green,
  RGB(u32, u32, u32),
  // HSV(u32, u32, u32),
  // HSL(u32, u32, u32),
  // CMY(u32, u32, u32),
  // CMYK(u32, u32, u32, u32),
}

fn main() {
  let color = Color::RGB(122, 17, 40);
  println!("What color is it?");

  match color {
    Color::Red => println!("The color is Red!"),
    Color::Blue => println!("The color is Blue!"),
    Color::Green => println!("The color is Green!"),
    Color::RGB(r, g, b) => println!("Red: {}, green: {}, blue: {}", r, g, b),
  }
}