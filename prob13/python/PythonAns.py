import sys

def main():
    number_strs = sys.stdin.readlines()
    numbers = [int(num[:15]) for num in number_strs]
    print(str(sum(numbers))[:10])

if __name__ == '__main__':
    main()
