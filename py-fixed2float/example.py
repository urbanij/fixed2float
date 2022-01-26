import fixed2float as f2f

print(f2f.to_fixed(1.25, 1, 3))
print(f2f.to_fixed(1.25, 1, 3).val)

print(f2f.to_float(0x7F, 8, 2, 6))

print(f2f.to_fixed(1 / 1.4375, 1, 15).val)
