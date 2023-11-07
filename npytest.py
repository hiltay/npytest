import numpy as np
import struct


# def write_array(array):
#     """
#     Write an array to an NPY file, including a header.
#
#     If the array is neither C-contiguous nor Fortran-contiguous AND the
#     file_like object is not a real file object, this function will have to
#     copy data in memory.
#
#     Parameters
#     ----------
#     fp : file_like object
#         An open, writable file object, or similar object with a
#         ``.write()`` method.
#     array : ndarray
#         The array to write to disk.
#     version : (int, int) or None, optional
#         The version number of the format. None means use the oldest
#         supported version that is able to store the data.  Default: None
#     allow_pickle : bool, optional
#         Whether to allow writing pickled data. Default: True
#     pickle_kwargs : dict, optional
#         Additional keyword arguments to pass to pickle.dump, excluding
#         'protocol'. These are only useful when pickling objects in object
#         arrays on Python 3 to Python 2 compatible format.
#
#     Raises
#     ------
#     ValueError
#         If the array cannot be persisted. This includes the case of
#         allow_pickle=False and array being an object array.
#     Various other errors
#         If the array contains Python objects as part of its dtype, the
#         process of pickling them may raise various errors if the objects
#         are not picklable.
#
#     """
#
#     if array.itemsize == 0:
#         buffersize = 0
#     else:
#         # Set buffer size to 16 MiB to hide the Python loop overhead.
#         buffersize = max(16 * 1024 ** 2 // array.itemsize, 1)
#
#     dtype_class = type(array.dtype)
#
#     for chunk in numpy.nditer(
#             array, flags=['external_loop', 'buffered', 'zerosize_ok'],
#             buffersize=buffersize, order='C'):
#         print(chunk.tobytes("C"))
#         for i in chunk.tobytes("C"):
#             print(hex(i))
#             print(format(i, "016b"))
#         # fp.write(chunk.tobytes('C'))
#
#
arr = np.array(
    [1.868,]
)
print(arr.dtype)
# arr = arr.astype("float16")
arr = arr.astype("float32")
# write_array(arr)
np.save("test.npy",arr)
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