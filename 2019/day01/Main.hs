import System.IO
import System.Environment

getMass :: String -> [Int]
getMass xs = map read (words xs)

getSumFuel :: String -> String
getSumFuel xs = show $ sum $ map calculateFuel $ getMass xs
                  where calculateFuel = (\x -> x `div` 3 - 2)

main :: IO()
main = do
  fileName <- getArgs
  file <- readFile $ head fileName
  putStrLn $ getSumFuel file
