use std::fs::File;
use std::io::{BufReader,Read};

fn main(){
    let file = File::open("./example.txt").expect("Failed to open file");
    let mut buf_reader = BufReader::new(file);

    let mut buffer = String::new();
    buf_reader.read_to_string(&mut buffer).expect("Failer to read from BufReader");
    println!("{}",buffer);
}