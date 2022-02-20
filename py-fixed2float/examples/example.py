import fixed2float
from fixed2float import Fx, to_Fx, to_float

print(f"v{fixed2float.__version__}")

fp1 = to_Fx(1.25, 4, 7, True)
fp2 = to_Fx(0.25, 4, 7, True)


print(f"{fp1=}, {fp1.val=}, {fp1.eval()=}")
print(f"{(fp1 + fp1).eval()=}")

print(f"{(fp1 - fp2).eval()=}")

print(f"{(fp1 * fp2).eval()=}")

print(f"{(fp1 << 1)=}, {(fp1 << 1).eval()=}")
print(f"{(fp1 >> 2)=}, {(fp1 >> 2).eval()=}")


print(to_float(0x7F, 8, 2, 6))

print(to_Fx(1 / 1.4375, 1, 16, True).val)

print(to_Fx(0, 1, 5, True).val)

fp3 = Fx(40, 3, 8)
print(fp3, fp3.as_str())
