from .fixed2float import to_float, to_float_str

__ANSI_RESET_COLOR = "\033[0m"
__ANSI_BLACK = "\033[37;40m"  # non bold, black background, white foreground
__ANSI_MAGENTA = "\u001b[45m"  # non bold, magenta background, black foreground


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
        repr = f"{__ANSI_MAGENTA}{bits[:m]}{__ANSI_BLACK}{bits[m:]}{__ANSI_RESET_COLOR}{dots}"
        return {
            "val": ans[0],
            "is_exact": ans[1],
            "repr": repr,
        }
