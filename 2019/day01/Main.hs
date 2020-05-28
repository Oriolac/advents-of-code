import System.IO
import System.Environment

calculateFuel :: Int -> Int
calculateFuel x = (div x 3) - 2

getMass :: String -> [Int]
getMass xs = map read (words xs)

getSumFuel :: String -> String
getSumFuel xs = show $ sum $ map calculateFuel $ getMass xs

main :: IO()
main = do
  fileName <- getArgs
  file <- readFile $ head fileName
  let summary = getSumFuel $ file
  putStrLn summary