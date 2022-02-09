import fixed2float as f2f

print(f"v{f2f.__version__}")

fp1 = f2f.to_Fx(1.25, 4, 7)
fp2 = f2f.to_Fx(0.25, 4, 7)


print(f"{fp1=}, {fp1.val=}, {fp1.eval()=}")
print(f"{(fp1 + fp1).eval()=}")

print(f"{(fp1 - fp2).eval()=}")

print(f"{(fp1 * fp2).eval()=}")

print(f"{(fp1 << 1)=}, {(fp1 << 1).eval()=}")
print(f"{(fp1 >> 2)=}, {(fp1 >> 2).eval()=}")


print(f2f.to_float(0x7F, 8, 2, 6))

print(f2f.to_Fx(1 / 1.4375, 1, 16).val)


fp3 = f2f.Fx(40, 3, 8)
print(fp3)
