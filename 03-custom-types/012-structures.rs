// 3 types of strusts
// unit, tuple, and c

#[derive(Debug)]
struct Person<'a> {
  name: &'a str,
  age: u8,
}

struct Nil;
struct Pair(i32, f32);
#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
  p1: Point,
  p2: Point,
}

fn main() {
  let name = "Peter";
  let age = 27;
  let peter = Person { name, age };

  println!("{:?}", peter);

  let point: Point = Point { x: 0.3, y: 0.4 };
  println!("point coordinates: ({}, {})", point.x, point.y);

  // not sure what the point of this was
  // let Point { x: my_x, y: my_y } = point;
  // let _rectangle = Rectangle {
  //   p1: Point { x: my_x, y: my_y },
  //   p2: point
  // };
  // println!("{:?}", _rectangle);

  let _nil = Nil;
  let pair = Pair(1, 0.1);
  println!("pair contains {:?} and {:?}", pair.0, pair.1);

  let Pair(integer, decimal) = pair;
  println!("pair contains {:?} and {:?}", integer, decimal);
}