def fibSequence(limit):
    first, second = 0, 1
    while second <= limit:
        yield second
        third = first + second
        first = second
        second = third

def main():
    print(sum(x for x in fibSequence(4000000) if x % 2 == 0))

if __name__ == '__main__':
    main()
