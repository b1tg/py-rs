import os
import ctypes
from ctypes import c_char_p

if __name__ == "__main__":
    lib_path = os.path.abspath("./target/debug/libpy_rs.dylib")

    # Load library
    lib = ctypes.cdll.LoadLibrary(lib_path)

    print("\nCall lib_test:")
    lib.lib_test()

    print("\nCall lib_get_str:")
    lib.lib_get_str.restype = c_char_p
    res = lib.lib_get_str()
    print("lib_get_str return; {}".format(res))
    
    print("\nCall lib_scan:")
    lib.lib_scan.restype = c_char_p
    lib.lib_scan.argtypes = [c_char_p]
    res = lib.lib_scan(b"src/lib.rs")
    print("lib_scan return; {}".format(res))
    
