import Data.IORef

main :: IO ()
main = do
  putStrLn "I'm going to calculate a sum, hang on a sec"
  totalRef <- newIORef (0 :: Int)
  let loop i
        | i > 100 = pure ()
        | otherwise = do
            oldTotal <- readIORef totalRef
            let newTotal = oldTotal + i
            writeIORef totalRef $! newTotal
            loop $! i + 1
  loop 1
  total <- readIORef totalRef
  putStrLn $ "The total is " ++ show total
