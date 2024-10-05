import sys

file = sys.argv[1]

b = bytearray([1, 2, 3, 4, 5])

with open(file=file, mode="wb") as f:
    f.write(b)