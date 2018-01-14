use List::*;

enum List {
  Cons(u32, Box<List>),
  Nil,
}