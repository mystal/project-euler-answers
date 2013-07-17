main = do
    let fibs = takeWhile (<= 4000000) fib
    print $ sum [x | x <- fibs, x `mod` 2 == 0]

fib = 0 : 1 : zipWith (+) fib (tail fib)
