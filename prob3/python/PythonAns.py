import math
import sys

def primeCandidates(n):
    top = n // 2
    return range(2, top + 1)

def primeFactors(n):
    factors = []
    cur = n
    foundFactor = True
    while foundFactor:
        foundFactor = False
        for x in primeCandidates(cur):
            if cur % x == 0:
                foundFactor = True
                factors.append(x)
                cur //= x
                break
    factors.append(cur)
    return factors

def main():
    print(primeFactors(int(sys.argv[1])))

if __name__ == '__main__':
    main()
