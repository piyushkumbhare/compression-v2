# This is a helper file that writes specified bytes to a target file

import sys

if len(sys.argv) != 2:
    print(f"Usage: {sys.argv[0]} FILE")
    exit(1)

file = sys.argv[1]

b = bytearray(range(256))

with open(file=file, mode="wb") as f:
    f.write(b)
