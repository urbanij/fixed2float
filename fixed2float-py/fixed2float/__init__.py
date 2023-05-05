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


from .fixed2float import PyFx as Fx
from .fixed2float import py_to_Fx as to_Fx

# variable names with leading double underscore gets mangles. it's the pythonic way of making things private.
from .fixed2float import version as __version
__version__ = __version()
