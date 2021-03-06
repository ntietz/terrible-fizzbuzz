# Copyright 2021 Nicholas Tietz-Sokolsky, All Rights Reserved.
#
# There is no open-source license available for this code because I truly do
# not want anyone to use this in production. That would be awful.

def numbers(num=1):
    yield num
    yield from numbers(num + 1)


def fizz(gen):
    yield next(gen)
    yield next(gen)
    next(gen); yield "fizz"
    yield from fizz(gen)

def buzz(gen):
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    next(gen); yield "buzz"
    yield from buzz(gen)

def fizzbuzz(gen):
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    yield next(gen)
    next(gen); yield "fizzbuzz"
    yield from fizzbuzz(gen)

def main():
    gen = numbers()
    gen = fizz(gen)
    gen = buzz(gen)
    gen = fizzbuzz(gen)

    for num in range(1, 101):
        print(next(gen))

# detect if we're being used as a library or not
# because surely this will be used as a library.
# million dollar ideas here.
if __name__ == "__main__":
    main()



