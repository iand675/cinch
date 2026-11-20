
  -- | Model decimal validity and value independently with bounded scalar byte operations.
  scalar-decimal :: Bytes -> Nat -> Nat -> Nat
  scalar-decimal input at total = if at < bytes-len input then
    let byte = bytes-get input at in
    if byte >= 48 && byte <= 57 then scalar-decimal input (at + 1) (total * 10 + (byte - 48)) else 100000000
    else total
  -- | Compare exact decimal and newline classification for one current runtime word.
  word-case :: Bytes -> Nat -> Bool
  word-case input byte = if bytes-len input >= 8 then
    let word = bytes-read-le64 input 0 in let expected = scalar-decimal input 0 0 in
    if expected <= 100000000 then eq-u64 (decimal-eight word) (wrapping-i64-to-u64 (nat-to-i64 expected)) && (ne-u8 (binary-linebreak-eight word) 0 == (byte == 13 || byte == 10)) else False
    else False
  -- | Mutate each of 256 possible byte values while retaining all seven other known decimal lanes.
  inner :: Bytes -> Nat -> Nat -> Bool
  inner input lane byte = if lane < 8 && bytes-len input >= 8 then
    if byte < 256 then let written = bytes-set input lane byte in if word-case input byte then inner input lane (byte + 1) else False
    else let restored = bytes-set input lane (49 + lane) in True
    else False
  -- | Visit every byte lane with a constant-stack traversal and restore the original word between lanes.
  outer :: Bytes -> Nat -> Bool
  outer input lane = if lane < 8 then if inner input lane 0 then outer input (lane + 1) else False else True
  -- | Return a nonzero result for any independent production-kernel disagreement.
  main :: Int
  main = if outer (bytes-from-list [49,50,51,52,53,54,55,56]) 0 then 0 else 1
