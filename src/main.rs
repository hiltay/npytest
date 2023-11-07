use ndarray_npy;
use std::alloc;
use std::fs::{self, File};
use std::io::{self, *};

const SIGN_M: u32 = 0x80000000;
const EXP_M: u32 = 0x7f800000;
const SIG_M: u32 = 0x007fffff;

/// 是否切换为定点表示
fn use_fixed_point(num: f32) -> bool {
    num > 0.0 && num < 1.0
}
///     1          2        3         4         5         6         7
/// xxxx xxxx xxxx xx|xx xxxx xxxx xxxx| xxxx xxxx xxxx xx|xx xxxx xxxx xxxx
/// 指数位4
/// 1110表示使用定点表示，整数部分为0
/// 1111表示异常数据（nan）
/// 小数位10
fn trans_f32_to_half_float(nums: [f32; 4]) -> [u8; 7] {
    let mut bund: [u8; 7] = [0; 7];

    if use_fixed_point(nums[0]) {// TODO
    } else {
        let num1 = nums[0].to_bits();
        bund[0] += (((((num1 & EXP_M) >> 23) - 127) & 0x0000000f) << 4) as u8; // 超出范围转换会失败？
        bund[0] += ((num1 & SIG_M) >> 19) as u8;
        bund[1] += (((num1 & SIG_M) & 0x0007e000) >> 11) as u8;
    }
    if use_fixed_point(nums[1]) {// TODO
    } else {
        let num2 = nums[1].to_bits();
        bund[1] += (((((num2 & EXP_M) >> 23) - 127) & 0x0000000f) >> 2) as u8;
        bund[2] += (((((num2 & EXP_M) >> 23) - 127) & 0x00000003) << 6) as u8;
        bund[2] += (((num2 & SIG_M) & 0x007e0000) >> 17) as u8;
        bund[3] += (((num2 & SIG_M) & 0x0001e000) >> 9) as u8;
    }
    if use_fixed_point(nums[2]) {
        // TODO
    } else {
        let num3 = nums[2].to_bits();
        bund[3] += ((((num3 & EXP_M) >> 23) - 127) & 0x0000000f) as u8;
        bund[4] += (((num3 & SIG_M) & 0x007f8000) >> 15) as u8;
        bund[5] += (((num3 & SIG_M) & 0x00006000) >> 7) as u8;
    }

    if use_fixed_point(nums[3]) {
        let num4 = ((nums[3] * 1000.0) as u16);
        // sign set to 1110
        bund[5] += 0x38;
        bund[5] += ((num4 & 0x0300) >> 8) as u8;
        bund[6] += (num4 & 0x00ff) as u8
    } else {
        let num4 = nums[3].to_bits();
        bund[5] += (((((num4 & EXP_M) >> 23) - 127) & 0x0000000f) << 2) as u8;
        bund[5] += ((num4 & SIG_M) >> 21) as u8;
        bund[6] += (((num4 & SIG_M) & 0x001fe000) >> 13) as u8;
    }
    bund
}

fn main() {
    let nums = [1.868, 47.299, 140.36, 1.058];
    let bund = trans_f32_to_half_float(nums);
    println!("{:?}", bund);

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
