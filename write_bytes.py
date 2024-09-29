import sys

file = sys.argv[1]

b = bytearray(range(256))

b.remove(65)

with open(file=file, mode="wb") as f:
    f.write(b)