import Data.List
import System.Environment

main = do
    args <- getArgs
    let num = (read . head) args :: Int
    print $ primeFactors num

primeFactors :: Int -> [Int]
primeFactors n = let candidates = primeCandidates n in
    case find  (\x -> 0 == (mod n x)) candidates of
        Just x -> x : (primeFactors (div n x)) --Found a prime factor, recurse
        Nothing -> [n] --No factors found, this is a prime number

primeCandidates :: Int -> [Int]
primeCandidates n = let top = (floor . (\x -> x / 2) . fromIntegral) n in
    [2 .. top]
