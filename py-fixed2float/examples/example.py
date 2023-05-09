import fixed2float
from fixed2float import Fx, to_Fx, to_float

print(f"v{fixed2float.__version__}")

fx1 = to_Fx(1.25, 4, 7, True)
fx2 = to_Fx(0.25, 4, 7, True)


print(f"{fx1=}, {fx1.val=}, {fx1.eval()=}")
print(f"{(fx1 + fx1).eval()=}")

print(f"{(fx1 - fx2).eval()=}")

print(f"{(fx1 * fx2).eval()=}")

print(f"{(fx1 << 1)=}, {(fx1 << 1).eval()=}")
print(f"{(fx1 >> 2)=}, {(fx1 >> 2).eval()=}")


print(to_float(0x7F, 8, 2, 6))

print(to_Fx(1 / 1.4375, 1, 16, True).val)

print(to_Fx(0, 1, 5, True).val)

fx3 = Fx(40, 3, 8)
print(fx3, fx3.as_str())
