use ndarray_npy;
use std::alloc;
use std::fs::{self, File};
use std::io::{self, *};

const sign_m: u32 = 0x80000000;
const exp_m: u32 = 0x7f800000;
const sig_m: u32 = 0x007fffff;

fn main() {
    let mut bund:[u8; 14] = [0;14];
    let test_num: f32 = 1.868; // 0x3fef1aa0
    let num_bits = test_num.to_bits();
    println!("{}",num_bits & sign_m);
    // (((test_num.to_bits() & exp_m)>>23)-127)<<4 as u8
    println!("{}",(((num_bits & exp_m)>>23)-127)<<4 as u8);
    println!("{}",(num_bits & sig_m)>>19);
    // (num_bits & sig_m)>>19 as u8;

    bund[0]+= ((((test_num.to_bits() & exp_m)>>23)-127)<<4) as u8;
    bund[0] += ((num_bits & sig_m)>>19) as u8;
    bund[1] += (((num_bits & sig_m) & 0x0007ffff) >>13) as u8;
    println!("{:?}",bund);
    // expect: 0b 0000 1101111000
    //            0000 1101111000
    // let mut npyfile = File::open("./test.npy").unwrap();
    // let mut buffer = Vec::new();
    // let total_bytes = npyfile.read_to_end(&mut buffer).unwrap();
    // println!("{}", total_bytes);
    // println!("{:?}",buffer.clone());
    // for i in buffer {
    // println!("{:?},{:#04x}", i as char, i);
    // }
    // println!("{:?}",buffer.iter().map(|v| *v as char).collect::<Vec<char>>() );
    // println!("{:?}",buffer.iter().map(|v| *v as char).collect::<String>() );
}
