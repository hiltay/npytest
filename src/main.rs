use ndarray_npy;
use std::fs::{self, File};
use std::io::{self, *};
/// 220.3
/// 0x cd4c5c43
/// 
fn main() {
    let mut npyfile = File::open("./test.npy").unwrap();
    let mut buffer = Vec::new();
    let total_bytes = npyfile.read_to_end(&mut buffer).unwrap();
    println!("{}", total_bytes);
    // println!("{:?}",buffer.clone());
    for i in buffer {
        println!("{:?},{:#04x}", i as char, i);
    }
    // println!("{:?}",buffer.iter().map(|v| *v as char).collect::<Vec<char>>() );
    // println!("{:?}",buffer.iter().map(|v| *v as char).collect::<String>() );
}
