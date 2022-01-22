from .fixed2float import to_float

__ANSI_RESET_COLOR = "\033[0m"
__ANSI_BLACK = "\033[1;37;40m"
__ANSI_COLOR_CYAN = "\x1b[36m"


def __get_bin(x, n):
    return format(x, "b").zfill(n)

# def to_float(bits, m, n) -> float:
#     return __fixed2float.to_float(bits, m, n)


def to_fixed(x, m, n):
    from .fixed2float import to_fixed
    
    ans = to_fixed(x, m, n)
    if ans == None:
        return
    else:    
        is_exact = ans[1]
        dots = "..." if not is_exact else ""
        bits = __get_bin(ans[0], m + n)
        repr = f"{__ANSI_COLOR_CYAN}{bits[:m]}{__ANSI_BLACK}{bits[m:]}{__ANSI_RESET_COLOR}{dots}"
        return {
            "val": ans[0],
            "is_exact": ans[1],
            "repr": repr,
        }
