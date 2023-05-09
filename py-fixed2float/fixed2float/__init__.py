# from .fixed2float import to_float, to_float_str
# from .fixed2float import Fx
# from .fixed2float import version as __version

# __version__ = __version()

# # variable names with leading double underscore gets mangles. it's the pythonic way of making things private.


# def to_Fx(x, m, b, round):
#     from .fixed2float import to_Fx as _to_Fx

#     return _to_Fx(x, m, b, round)


# __all__ = [
#     "to_Fx",
#     "to_float",
#     "Fx",
# ]


from .fixed2float import py_from_double as from_double
from .fixed2float import py_from_bits as from_bits
from .fixed2float import py_to_Fx as to_Fx
from .fixed2float import PyFx as Fx

# variable names with leading double underscore gets mangles. it's the pythonic way of making things private.
from .fixed2float import version as __version
__version__ = __version()


# class Fixed(Fx):
#   def __init__(self, v, m, b, round=False):
#     if type(v) == int:
#       from_bits(v, m, b)
#     elif type(v) == float:
#       from_double(v, m, b, round)
#     else:
#       raise Exception("Wrong type")   
