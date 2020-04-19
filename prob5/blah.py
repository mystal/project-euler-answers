import sys

def main(n):
  x = n
  while True:
    good = True
    for i in xrange(1, n+1):
      if x % i != 0:
        good = False
        break
    if good:
      print n, x
      return
    x += n


if __name__ == '__main__':
  n = int(sys.argv[1])
  for i in xrange(1, n + 1):
    main(i)
