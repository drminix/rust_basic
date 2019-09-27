extern crate image;
extern crate num;

use num::One;
use std::str::FromStr;
use std::ops::AddAssign;

///Generic function which supports type T which implements FromStr traits
fn add_one<T: FromStr + AddAssign + One>(s: &str) -> Option<T> {
  match T::from_str(s) {
    Ok(l) => {
      let mut t : T = l;
      t += T::one();
      Some(t)
    }
    _ => None
  }
}

fn main() {
    println!("Hello, world!");
    let a :&str = "1.0";
    let b :f64 = add_one(a).expect("Doesn't work");
    println!("value is {}",b)
}
