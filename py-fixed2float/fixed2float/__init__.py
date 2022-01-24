from .fixed2float import to_float, to_float_str
from .fixed2float import version as __version

__version__ = __version()

# variable names with leading double underscore gets mangles. it's the pythonic way of making things private.

class __FixedPoint:
    def __init__(self, ans, m, n):
        (self.val, self.is_exact) = ans
        self.__m = m
        self.__n = n

    def _calc_bits(self) -> str:    
        (val, m, n) = (self.val, self.__m, self.__n)
        bits = _get_bin(val, m + n)
        return bits

    def __repr__(self):
        ANSI_RESET_COLOR = "\033[0m"
        ANSI_BLACK = "\033[37;40m"  # non bold, black background, white foreground
        ANSI_MAGENTA = "\u001b[45m"  # non bold, magenta background, black foreground
        m = self.__m
        dots = "..." if not self.is_exact else ""
        return f"{ANSI_MAGENTA}{self._calc_bits()[:m]}{ANSI_BLACK}{self._calc_bits()[m:]}{ANSI_RESET_COLOR}{dots}"


def _get_bin(x, n):
    return format(x, "b").zfill(n)


def to_fixed(x, m, n):
    from .fixed2float import to_fixed

    ans = to_fixed(x, m, n)
    if ans == None:
        return
    else:
        return __FixedPoint(ans, m, n)
