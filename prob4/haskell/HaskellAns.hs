import System.Environment

main = do
    args <- getArgs
    let numDigits = (read . head) args :: Int
    print $ largestPalindromeProduct numDigits

largestPalindromeProduct :: Int -> Int
largestPalindromeProduct n = palProdHelper 0 $ genLists (nDigitLimits n)

isPalindrome :: String -> Bool
isPalindrome s = if (length s `mod` 2 == 0) then -- TODO: Remove branch and calculate drop length?
        let topHalf = take ((length s) `div` 2) s
            bottomHalf = drop ((length s) `div` 2) s
        in  topHalf == (reverse bottomHalf)
    else
        let topHalf = take ((length s) `div` 2) s
            bottomHalf = drop (((length s) `div` 2) + 1) s
        in  topHalf == (reverse bottomHalf)

palProdHelper :: Int -> [(Int, [Int])] -> Int
--palProdHelper :: [(Int, Int)] -> Int
palProdHelper n [] = n
palProdHelper n ((x, []):ns) = palProdHelper n ns
palProdHelper n ((x, y:ys):ns) = if (isPalindrome $ show prod) && prod > n then
        palProdHelper prod ns --optimization?
        --palProdHelper prod ((x, ys):ns)
    else
        palProdHelper n ((x, ys):ns)
    where prod = x * y

genLists :: (Int, Int) -> [(Int, [Int])]
genLists (bottom, top) = [(x, [y | y <- [x, x-1..bottom]]) | x <- [top, top-1..bottom+1]]
--genLists :: (Int, Int) -> [(Int, Int)]
--genLists (bottom, top) = zip l (tail l)
--    where l = concatMap (replicate 2) [top, top-1..bottom]

nDigitLimits :: Int -> (Int, Int)
nDigitLimits n = (10 ^ (n - 1), (10 ^ n) - 1)
