// use ndarray_npy;
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, *};
// const SIGN_M: u32 = 0x80000000;
const EXP_M: u32 = 0x7f800000;
const SIG_M: u32 = 0x007fffff;

/// 是否切换为定点表示
fn use_fixed_point(num: f32) -> bool {
    num < 1.0
}
///     1          2        3         4         5         6         7
/// xxxx xxxx xxxx xx|xx xxxx xxxx xxxx|xxxx xxxx xxxx xx|xx xxxx xxxx xxxx
/// 指数位4
/// 1110表示使用定点表示，整数部分为0
/// 1111表示异常数据（nan），整数部分为0
/// 小数位10
fn trans_f32_to_half_float(nums: Vec<f32>) -> [u8; 7] {
    let mut bund: [u8; 7] = [0; 7];

    if use_fixed_point(nums[0]) {
        let num1 = (nums[0] * 1000.0) as u16;
        bund[0] += 0xe0;
        bund[0] += ((num1 & 0x03c0) >> 6) as u8;
        bund[1] += ((num1 & 0x003f) << 2) as u8;
    // 处理nan
    } else if nums[0].is_nan() {
        bund[0] += 0xf0;
    } else {
        let num1 = nums[0].to_bits();

        bund[0] += (((((num1 & EXP_M) >> 23) - 127) & 0x0000000f) << 4) as u8;
        bund[0] += ((num1 & SIG_M) >> 19) as u8;
        bund[1] += (((num1 & SIG_M) & 0x0007e000) >> 11) as u8;
    }
    if use_fixed_point(nums[1]) {
        let num2 = (nums[1] * 1000.0) as u16;
        bund[1] += 0x03;
        bund[2] += 0x80;
        bund[2] += ((num2 & 0x03f0) >> 4) as u8;
        bund[3] += ((num2 & 0x000f) << 4) as u8;
    } else if nums[1].is_nan() {
        bund[1] += 0x03;
        bund[2] += 0xc0;
    } else {
        let num2 = nums[1].to_bits();

        bund[1] += (((((num2 & EXP_M) >> 23) - 127) & 0x0000000f) >> 2) as u8;
        bund[2] += (((((num2 & EXP_M) >> 23) - 127) & 0x00000003) << 6) as u8;
        bund[2] += (((num2 & SIG_M) & 0x007e0000) >> 17) as u8;
        bund[3] += (((num2 & SIG_M) & 0x0001e000) >> 9) as u8;
    }
    if use_fixed_point(nums[2]) {
        let num3 = (nums[2] * 1000.0) as u16;
        bund[3] += 0x0e;
        bund[4] += ((num3 & 0x03fc) >> 2) as u8;
        bund[5] += ((num3 & 0x0003) << 6) as u8;
    } else if nums[2].is_nan() {
        bund[3] += 0x0f;
    } else {
        let num3 = nums[2].to_bits();
        bund[3] += ((((num3 & EXP_M) >> 23) - 127) & 0x0000000f) as u8;
        bund[4] += (((num3 & SIG_M) & 0x007f8000) >> 15) as u8;
        bund[5] += (((num3 & SIG_M) & 0x00006000) >> 7) as u8;
    }

    if use_fixed_point(nums[3]) {
        let num4 = (nums[3] * 1000.0) as u16;
        // sign set to 1110
        bund[5] += 0x38;
        bund[5] += ((num4 & 0x0300) >> 8) as u8;
        bund[6] += (num4 & 0x00ff) as u8
    } else if nums[3].is_nan() {
        bund[5] += 0x3c;
    } else {
        let num4 = nums[3].to_bits();
        bund[5] += (((((num4 & EXP_M) >> 23) - 127) & 0x0000000f) << 2) as u8;
        bund[5] += ((num4 & SIG_M) >> 21) as u8;
        bund[6] += (((num4 & SIG_M) & 0x001fe000) >> 13) as u8;
    }
    bund
}

struct Npyfile {
    // 头部，6字节，分别是：\x93 N U M P Y
    header: [u8; 6],
    // 主版本号，例如 \x01
    major_version: u8,
    // 次版本号，例如 \x00
    minor_version: u8,
    // 2 个字节形成一个小端无符号短整型：头数据 HEADER_LEN 的长度。
    header_len: [u8; 2],
    // 它是一个 ASCII 字符串，其中包含字典的 Python 文字表达式。
    // 它以换行符 ( \n ) 结尾，并用空格 ( \x20 ) 填充，
    // 使 len(magic string) + 2 + len(length) + HEADER_LEN 的总数可被 64 整除，以便对齐。
    header_data: Vec<u8>,
    // 数据部分
    data: Vec<u8>,
}
fn parse_npy_file(bytes: &Vec<u8>) -> (Vec<usize>, usize, Vec<u8>) {
    let header_data_len = ((bytes[9] as u16) << 8) + (bytes[8] as u16);
    let header_data = &bytes[10..10 + header_data_len as usize];
    // 获取shape
    let s = header_data.iter().map(|v| *v as char).collect::<String>();
    let re = Regex::new(r"shape.*\((.*)\)").unwrap();
    let caps = re.captures(&s).unwrap();

    let shape = match caps.get(1) {
        Some(sh) => sh.as_str(),
        None => panic!("无法解析npy中的shape"),
    };
    println!("shape:{}", shape);
    let mut shape_vec = Vec::new();
    for s in shape.split(',') {
        if s.trim().len() > 0 {
            shape_vec.push(s.trim().parse::<usize>().unwrap())
        }
    }
    let total_data_num = if shape_vec.len() == 1 {
        shape_vec[0]
    } else if shape_vec.len() == 2 {
        shape_vec[0] * shape_vec[1]
    } else {
        panic!("只支持解析1D/2D数组")
    };

    // println!("总数据：{}", total_data_num);
    let data = &bytes[(10 + header_data_len) as usize..];
    // println!("{:?}", data);
    let mut result = Vec::new();
    for i in data {
        result.push(*i);
    }
    (shape_vec, total_data_num, result)
}

/// 读取npy文件，将其转换为half_float表示
/// 写入二进制bin文件
/// 其中：
/// 前8位为版本号
/// 之后的32位为矩阵的形状，目前只支持2D矩阵
/// - 前16位为行，如果是1D矩阵，则此值永远为1
/// - 后16位为列
/// 之后为数据段，它们永远是56的整数倍

fn trans_main(filename: &str) {
    let mut npyfile = File::open(filename).unwrap();
    let mut buffer = Vec::new();
    let total_bytes = npyfile.read_to_end(&mut buffer).unwrap();
    // println!("{}", total_bytes);
    // println!("{:?}", buffer.clone());
    let (shape_vec, total_data_num, result) = parse_npy_file(&buffer);
    let mut parsed_data = Vec::new();
    let mut container = Vec::new();
    for i in 0..total_data_num {
        // little endian
        let data = &result[i * 4..i * 4 + 4];
        let bytes: [u8; 4] = data.try_into().expect("slice with incorrect length");
        let num = u32::from_le_bytes(bytes);
        // println!("{}", num);
        unsafe {
            let fnum = std::mem::transmute::<u32, f32>(num);
            container.push(fnum);
        }
        // println!("{:?}", container);
        if container.len() == 4 {
            parsed_data.push(container.clone());
            container.clear();
        }
    }
    if container.len() > 0 {
        for _ in 0..4 - container.len() {
            container.push(0.0)
        }
        parsed_data.push(container);
    }
    // println!("{:?}", parsed_data);
    let mut file = File::create(format!("{}.bin", filename.split_once(".").unwrap().0)).unwrap();
    // 定义一个版本，当前为1
    let version: [u8; 1] = [1];

    // shape u32
    //  row u16  col u16
    // 0x rrrr cccc
    let mut shape: u32 = 0;
    if shape_vec.len() == 1 {
        shape += 1 << 16;
        shape += (shape_vec[0] as u32) & 0x0000ffff;
    } else {
        shape += (shape_vec[0] as u32) << 16;
        shape += (shape_vec[1] as u32) & 0x0000ffff;
    };
    file.write_all(&version).unwrap();
    file.write_all(&shape.to_le_bytes()).unwrap();
    // group
    for bundle in parsed_data {
        let bund = trans_f32_to_half_float(bundle);
        // println!("bund {:?}", bund);
        file.write_all(&bund).unwrap();
    }
}

fn main() {

    trans_main("test.npy")
}
