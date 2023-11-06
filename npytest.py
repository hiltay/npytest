import numpy as np
import numpy
import struct


def write_array(array):
    """
    Write an array to an NPY file, including a header.

    If the array is neither C-contiguous nor Fortran-contiguous AND the
    file_like object is not a real file object, this function will have to
    copy data in memory.

    Parameters
    ----------
    fp : file_like object
        An open, writable file object, or similar object with a
        ``.write()`` method.
    array : ndarray
        The array to write to disk.
    version : (int, int) or None, optional
        The version number of the format. None means use the oldest
        supported version that is able to store the data.  Default: None
    allow_pickle : bool, optional
        Whether to allow writing pickled data. Default: True
    pickle_kwargs : dict, optional
        Additional keyword arguments to pass to pickle.dump, excluding
        'protocol'. These are only useful when pickling objects in object
        arrays on Python 3 to Python 2 compatible format.

    Raises
    ------
    ValueError
        If the array cannot be persisted. This includes the case of
        allow_pickle=False and array being an object array.
    Various other errors
        If the array contains Python objects as part of its dtype, the
        process of pickling them may raise various errors if the objects
        are not picklable.

    """

    if array.itemsize == 0:
        buffersize = 0
    else:
        # Set buffer size to 16 MiB to hide the Python loop overhead.
        buffersize = max(16 * 1024 ** 2 // array.itemsize, 1)

    dtype_class = type(array.dtype)

    for chunk in numpy.nditer(
            array, flags=['external_loop', 'buffered', 'zerosize_ok'],
            buffersize=buffersize, order='C'):
        print(chunk.tobytes("C"))
        for i in chunk.tobytes("C"):
            print(hex(i))
            print(format(i, "016b"))
        # fp.write(chunk.tobytes('C'))


arr = np.array(
    [[220.3,220.7,219.8]
    for i in range(100)]
)
print(arr.dtype)
arr = arr.astype("float16")
# arr = arr.astype("float32")
write_array(arr)
np.save("test.npy",arr)


# def print_float(num):
#     print(num)
#     # 将浮点数转换为字节流
#     b = struct.pack("f", num)
#     # 将字节流转换为整数
#     i = struct.unpack("i", b)[0]
#     print(i)
#     # 将整数转换为二进制字符串
#     return bin(i),hex(i)
# def sign_exp_sig(num):
#     ...
#
# # 0b 0 10000000 10010010 01101110 1001100
# # sign - 1bit
# # exp - 5bit
# # sig - 2bit
# print(print_float(3.144))