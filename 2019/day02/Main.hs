import System.IO
import System.Environment

import Data.Vector (Vector, (//))

type Code = Vector Int

data Op = Add | Mult | Halt

intToOp :: Int -> Op
intToOp 1 = Add
intToOp 2 = Mult
intToOp 99 = Halt
intToOp x = error $ show x ++ "is not declared to be an operation."

main :: IO()
main = do
  args <- getArgs
  fileText <- readFile $ head args
  let code = read ("[" ++ fileText ++ "]") :: Code
  putStrLn $ show st
