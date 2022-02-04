import fixed2float as f2f

# print(f"{f2f.__version__}")

fp1 = f2f.to_fixed(1.25, 4, 3)
fp2 = f2f.to_fixed(0.25, 4, 3)


print(f"{fp1=}, {fp1.val=}, {fp1.eval()=}")
print(f"{(fp1 + fp1).eval()=}")

print(f"{(fp1 - fp2).eval()=}")

print(f"{(fp1 * fp2).eval()=}")

print(f"{(fp1 << 1)=}, {(fp1 << 1).eval()=}")
print(f"{(fp1 >> 2)=}, {(fp1 >> 2).eval()=}")


print(f2f.to_float(0x7F, 8, 2, 6))

print(f2f.to_fixed(1 / 1.4375, 1, 15).val)


fp3 = f2f.FixedPoint(40, 3, 5)
