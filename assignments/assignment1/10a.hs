data Colour = Red | Yellow | Blue

sayColour colour =
  case colour of
    Red -> "red"
    Yellow -> "yellow"

main = putStrLn (sayColour Blue)
