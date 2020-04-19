from __future__ import print_function

import math
import sys

def num_digits_of(n):
    return int(math.log10(n)) + 1

def main():
    if len(sys.argv) != 2:
        print("Usage:\n{} <num_digits>".format(sys.argv[0]))
        return

    num_digits = int(sys.argv[1])

    if num_digits <= 1:
        print(0)
        return

    prev = 0
    cur = 1

    index = 1

    while num_digits_of(cur) < num_digits:
        next = prev + cur
        prev = cur
        cur = next
        index += 1

    print("Num: {}".format(cur))
    print("Index: {}".format(index))

if __name__ == '__main__':
    main()
