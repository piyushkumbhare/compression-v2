import sys

file = sys.argv[1]

b = bytearray([97, 98, 0])

with open(file=file, mode="wb") as f:
    f.write(b)