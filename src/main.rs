extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

mod sudoku;

fn main() {
    // println!("new");
    // println!("{}", sudoku::Working::new());

    let path = Path::new("samples/3.json");
    // let mut file = File::create(&path).unwrap();
    // file.write_all(
    //     serde_json::to_string(&sudoku::Serializable::new())
    //         .unwrap()
    //         .as_bytes(),
    // )
    // .unwrap();
    let mut file = File::open(&path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut sdk: sudoku::Working = serde_json::from_str::<sudoku::Serializable>(&contents)
        .unwrap()
        .into();
    println!("sample");
    println!("{}", sdk);
    println!("solved: {}", sdk.solve());
    println!("result:");
    println!("{}", sdk);
}
