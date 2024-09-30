import sys

file = sys.argv[1]

b = bytearray(range(256))

with open(file=file, mode="wb") as f:
    f.write(b)