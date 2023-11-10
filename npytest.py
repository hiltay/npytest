import numpy as np
import struct
# 2^13 2^12 2^11

arr = np.array([
    [i+(j/100) for i in range(32768)]
    for j in range(100)
]
)
print(arr.dtype)
# arr = arr.astype("float16")
arr = arr.astype("float32")
# write_array(arr)
np.save("test.npy", arr)
# i = np.load("b_i.npy")
# pf = np.load("b_pf.npy")
# print()


def get_float(num):
    # print(num)
    # 将浮点数转换为字节流
    b = struct.pack("f", num)
    # 将字节流转换为整数
    i = struct.unpack("i", b)[0]
    # print(bin(i), hex(i))
    # print(i)
    # 将整数转换为二进制字符串
    return i


def sign_exp_sig(num):
    sign_m = 0x80000000
    exp_m = 0x7f800000
    sig_m = 0x007fffff
    print(get_float(num) & sign_m)
    print(((get_float(num) & exp_m) >> 23)-127)
    print(bin(get_float(num) & sig_m))


    # 0b 0000 1101111000
#        0000 1101111000
# 0b 0 10000000 10010010 01101110 1001100
# 0b 0 11000000 00000000 00000000 0000000
# 0b 0 00111111 00000000 00000001 1111100
# 1 8 23
# print(get_float(1.868))
sign_exp_sig(1.868)
