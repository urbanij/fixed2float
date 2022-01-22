from .fixed2float import to_float

RESET_COLOR = "\033[0m"
ANSI_BLACK = "\033[1;37;40m"
ANSI_COLOR_CYAN = "\x1b[36m"


def get_bin(x, n):
    return format(x, "b").zfill(n)

def to_fixed(x, m, n, fancy=True) -> str:
    from .fixed2float import to_fixed
    
    ans = to_fixed(x, m, n)
    if ans == None:
        return
    else:    
        if fancy:
            is_exact = ans[1]
            dots = "..." if not is_exact else ""
            bits = get_bin(ans[0], m + n)
            return f"{ANSI_COLOR_CYAN}{bits[:m]}{ANSI_BLACK}{bits[m:]}{RESET_COLOR}{dots}"
        else:
            return f"{ans}"
