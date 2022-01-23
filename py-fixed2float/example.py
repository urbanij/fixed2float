import fixed2float as f2f

print(f2f.to_fixed(1.25, 1, 3)["repr"])

print(f2f.to_float(0x7F, 2, 6))

print(f2f.to_fixed(1 / 1.4375, 1, 15)["repr"])
