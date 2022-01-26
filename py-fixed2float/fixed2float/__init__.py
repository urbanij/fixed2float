from .fixed2float import to_float, to_float_str
from .fixed2float import version as __version

__version__ = __version()

# variable names with leading double underscore gets mangles. it's the pythonic way of making things private.

# def mask(n):
#     return (1 << n) - 1  # N-bits ALL ones


# class FixedPoint:
#     """Fixed Point class"""

#     def __init__(self, val, m, n, is_exact=True):
#         self.val = val
#         self.is_exact = is_exact
#         self.m = m
#         self.n = n

#     def _calc_bits(self) -> str:
#         (val, m, n) = (self.val, self.m, self.n)
#         bits = _get_bin(val, m + n)
#         return bits

#     def eval(self) -> float:
#         return to_float(self.val, self.m + self.n, self.m, self.n)

#     def __add__(self, other):
#         if self.m != other.m or self.n != other.n:
#             raise ("Can't add floating points in different formats")
#         return FixedPoint(self.val + other.val, self.m + 1, self.n)

#     def __sub__(self, other):
#         if self.eval() < other.eval():
#             raise NotImplemented()
#         if self.m != other.m or self.n != other.n:
#             raise ("Can't add floating points in different formats")
#         return FixedPoint(self.val - other.val, self.m, self.n)
    
#     def __lshift__(self, other):
#         if type(other) == int:
#             return FixedPoint((self.val << other) & mask(self.m + self.n), self.m, self.n)
#         raise TypeError()
    
#     def __rshift__(self, other):
#         if type(other) == int:
#             return FixedPoint((self.val >> other) & mask(self.m + self.n), self.m, self.n)
#         raise TypeError()

#     def __mul__(self, other):
#         if self.m != other.m or self.n != other.n:
#             print("sizes are different, make sure you know what you're doing.")
#         return FixedPoint(self.val * other.val, self.m + other.m, self.n + other.n)
    
#     def __getitem__(self, slice):
#         if type(slice) != slice:
#             print("Error, has to be a slice, e.g. [3:0]")
#             return
#         start, stop = slice.stop, slice.start # yes, reversed. that's also how verilog behaves


#     def __repr__(self):
#         ANSI_RESET_COLOR = "\033[0m"
#         ANSI_BLACK = "\033[37;40m"  # non bold, black background, white foreground
#         ANSI_MAGENTA = "\u001b[45m"  # non bold, magenta background, black foreground
#         m = self.m
#         dots = "..." if not self.is_exact else ""
#         return f"{ANSI_MAGENTA}{self._calc_bits()[:m]}{ANSI_BLACK}{self._calc_bits()[m:]}{ANSI_RESET_COLOR}{dots}"


# def _get_bin(x, n):
#     return format(x, "b").zfill(n)


def to_fixed(x, m, n):
    from .fixed2float import to_fixed as _to_fixed
    return _to_fixed(x, m, n)

#     ans = to_fixed(x, m, n)
#     if ans == None:
#         return
#     else:
#         return FixedPoint(ans[0], m, n, ans[1])


# __all__ = [
#     "to_fixed",
#     "to_float",
# ]
