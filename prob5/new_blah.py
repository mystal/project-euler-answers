import sys

def main(n):
  primes = []

  ans = 1
  for i in xrange(2, n + 1):
    x = i
    for prime in primes:
      if x % prime == 0:
        x /= prime
    if x > 1:
      primes.append(x)
      ans *= x
  print n, ans


if __name__ == '__main__':
  n = int(sys.argv[1])
  for i in xrange(1, n + 1):
    main(i)
