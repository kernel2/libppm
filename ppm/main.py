from ctypes import cdll
import os

lib_filename = "libembed.so";
lib_pathdirectory = os.path.dirname(os.path.abspath("target/release/"+lib_filename));

lib = cdll.LoadLibrary(lib_pathdirectory + "/" + lib_filename);
lib.toSeeIfPixelWorks();