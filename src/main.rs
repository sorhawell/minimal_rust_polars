use polars::prelude::*;

fn main() {
    let s = Series::new("hello polars", &[1, 2, 3]);
    println!("this is a polars series {:?}", s);
    println!("Hello, world!");
}
