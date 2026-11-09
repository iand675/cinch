# 7. Numbers, text, and bytes

Three number families and three text types cover the values that cross a boundary, and choosing the wrong one is the usual source of a silent wrap, a mangled character, or a conversion the checker refuses. This chapter names them, with their overflow policies, conversions, encoders and decimal formatting. Work in a file named `Main.cinch`:

```sh
cinch check Main.cinch
cinch run Main.cinch
```

Build the compiler and put it on `PATH` with [using the compiler](../reference/01-using-the-compiler.md); `check` typechecks and `run` executes.

**Prerequisites.** [Chapter 5](05-recursion-and-collections.md) for `Nat` and the obligations that `Dec` evidence discharges, and [chapter 6](06-options-results-and-validation.md) for `Option`, since every conversion that can lose information answers `Option`.

# Three number families

| Family | Types | Width | Overflow |
| --- | --- | --- | --- |
| Mathematical | `Nat`, `Int` | grows with the value | none, bounded by process resources |
| Machine integer | `U8`, `U16`, `U32`, `U64`, `I8`, `I16`, `I32`, `I64` | fixed per type | never implicit; the operation name chooses |
| Target-sized | `Usize`, `Isize` | the target's pointer width | never implicit; the operation name chooses |

`Nat` has no negative values and `Int` has both signs; both are boxed bignums, so `Nat` subtraction needs the `b <= a` evidence from chapter 5 while `Int` subtraction needs none. The machine types carry their bounds as named values:

| Type | `min-bound-` | `max-bound-` |
| --- | --- | --- |
| `U8`, `U16` | 0 | 255, 65535 |
| `U32`, `U64` | 0 | 4294967295, 18446744073709551615 |
| `I8`, `I16` | -128, -32768 | 127, 32767 |
| `I32`, `I64` | -2147483648, -9223372036854775808 | 2147483647, 9223372036854775807 |

`min-bound-u8`, `max-bound-i64` and the rest are declarations in `Data.Machine`, each refined to the value it states, so `nat-from-i64 max-bound-i64` discharges its range obligation with nothing written.

# Overflow policy is in the operation name

There is no `Add` instance for a machine type, because a class method has one result type and cannot say which overflow policy the caller meant. Each policy is its own name, so the arithmetic cannot silently wrap:

```cinch
  wrapped :: U8
  wrapped = wrapping-add-u8 255 1

  clamped :: U8
  clamped = saturating-add-u8 255 1

  checked :: Option U8
  checked = checked-add-u8 255 1
```

`checked-add-u8 :: U8 -> U8 -> Option U8` answers `None` when the exact sum does not fit, so the caller must handle the case. `wrapping-add-u8 255 1` is 0 by two's complement wrap, and `saturating-add-u8 255 1` is 255 because the result clamps at the bound. Every machine type has the same three families for addition, subtraction, multiplication, negation, division, remainder and the shifts, spelled `checked-add-i32`, `saturating-mul-u16`, `wrapping-sub-u64` and so on.

# Conversions have names, and each states what it needs

The mathematical conversions keep the value and state the fact they need; the `checked` forms answer `Option` instead.

| Name | Signature | Note |
| --- | --- | --- |
| `nat-to-int` | `(n :: Nat) -> refined (x == n) Int` | every `Nat` is an `Int` |
| `nat-from-int` | `using (0 <= x) => (x :: Int) -> refined (n == x) Nat` | needs non-negativity |
| `nat-to-i64` | `using (n <= 9223372036854775807) => (n :: Nat) -> refined (x == n) I64` | needs the range |
| `nat-from-i64` | `using (0 <= x) => (x :: I64) -> refined (n == x) Nat` | needs non-negativity |
| `int-to-i64` | `using (-9223372036854775808 <= x, x <= 9223372036854775807) => ... I64` | needs both bounds |
| `int-from-i64` | `(x :: I64) -> refined (r == x) Int` | always succeeds |
| `checked-nat-to-i64`, `checked-nat-from-i64`, `checked-int-to-i64` | `Option` result | `None` outside the range |

Between machine widths the pattern repeats. A widening conversion is a plain name that carries the source range into the result, such as `u8-to-i64 :: (x :: U8) -> refined (InRange 0 255 r && r == x) I64`. A narrowing conversion has three spellings: `checked-i64-to-u8 :: I64 -> Option U8`, `saturating-i64-to-u8 :: I64 -> U8`, and `wrapping-i64-to-u8 :: I64 -> U8`. The complete family is `checked-`, `saturating-` and `wrapping-` for every source and destination width pair, and the plain `source-to-destination` names for the widening pairs.

An ascription is not a conversion. `x :: U8` states a type and fails to check when `x` is not already a `U8`; `checked-i64-to-u8 x` is the operation that tests whether the value fits.

# Text is String, bytes are Bytes, names are Sym

`String` is text: a sequence of Unicode scalar values. `Bytes` is a buffer of bytes that may hold anything, including a sequence that is not valid UTF-8. `Sym` is an interned name: one word holding a handle to a process-wide cell, where equality is identity and ordering is lexicographic by name.

| Name | Module | Signature |
| --- | --- | --- |
| `string-to-bytes` | `Data.String` | `String -> Bytes` |
| `string-from-bytes` | `Data.String` | `Bytes -> Option String` |
| `string-length`, `string-byte-length` | `Data.String` | counts in characters, in UTF-8 bytes |
| `string-char-at`, `string-slice` | `Data.String` | `Option Char` at an index, then a substring |
| `string-concat`, `string-concat-all` | `Data.String` | pair, then list of strings |
| `string-repeat`, `pack`, `unpack` | `Data.String` | repetition, `List Char` conversions |
| `string-compare` | `Data.String` | `String -> String -> I64` |
| `intern`, `sym-name` | `Data.Sym` | `String -> Sym`, `Sym -> String` |

`string-to-bytes` encodes UTF-8 and `string-from-bytes` decodes, which is why the second one answers `Option`: a buffer that is not valid UTF-8 has no `String` value, and the type does not let you pretend otherwise. `string-length` and `string-byte-length` disagree as soon as a character costs more than one byte, so pick the one your rule is about.

A `Sym` literal is written with one apostrophe and no closing quote, so `'answer` is the interned name printed as `'answer`. A one-character literal in single quotes with a closing quote, `'a'`, is a `Char`. Two equal names are the same cell, so comparing `Sym` values costs one pointer test, and `'answer == intern "answer"` holds at run time.

# Construct bytes and index them with their refinements

Byte operations take their safety conditions as argument refinements rather than checking them at run time, and `bytes-new` supplies the length fact to the name it is bound to.

| Name | Signature |
| --- | --- |
| `bytes-new` | `(n :: Nat) -> refined (bytes-len out == n) Bytes` |
| `bytes-len` | `Bytes -> InRange 0 9223372036854775807 Nat` |
| `bytes-get` | `using (i < bytes-len b) => (b :: Bytes) -> (i :: Nat) -> InRange 0 255 Nat` |
| `bytes-set` | `using (i < bytes-len b) => (b :: Bytes) -> (i :: Nat) -> InRange 0 255 Nat -> Unit` |
| `bytes-from-list`, `bytes-from-u8s` | `List (InRange 0 255 Nat) -> Bytes`, `List U8 -> Bytes` |
| `bytes-copy-range` | `using (di + n <= bytes-len dst, si + n <= bytes-len src) => ... -> Unit` |
| `bytes-compare`, `sha256-bytes` | raw comparison, then the digest |

`bytes-get` returns a value refined to `0` through `255`, and `bytes-set` accepts only such a value, so a byte cannot leave its range in either direction. `bytes-set` returns `Unit`, and the language has no implicit sequencing, so each write is bound to a name:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  text :: String
  text = "Cinch"

  encoded :: Bytes
  encoded = string-to-bytes text

  built :: Bytes
  built = bytes-from-list [67, 105, 110, 99, 104]

  first-byte :: Option Nat
  first-byte = case decide (0 < bytes-len encoded) of
    Yes evidence -> Some (bytes-get encoded 0)
    No _ -> None

  pair :: Bytes
  pair =
    let buf = bytes-new 2 in
    let first = bytes-set buf 0 65 in
    let second = bytes-set buf 1 66 in
    buf

  symbol :: Sym
  symbol = intern text

  renamed :: String
  renamed = sym-name symbol

  main :: Int
  main =
    if string-length text == 5
      && string-byte-length text == 5
      && bytes-len encoded == 5
      && encoded == built
      && bytes-len pair == 2
      && renamed == "Cinch"
      then case first-byte of
        None -> 1
        Some value -> if value == 67 then 0 else 2
      else 3
```

`encoded` and `built` are equal because `"Cinch"` is five ASCII characters, one byte each. `first-byte` discharges `0 < bytes-len encoded` in its `Yes evidence` branch. `pair` never states that fact: `bytes-new 2` refines the bound name to `bytes-len buf == 2`, which is the evidence both writes need, and a buffer's length is fixed when it is made, so the fact survives the writes. `encoded == built` uses the byte-for-byte `Eq Bytes`.

`string-slice` takes two `Nat` bounds and `string-char-at` a character index, both answering `Option` where the index can be out of range.

# Decimal formatting

`Text.Show` holds the conversions from a number to its decimal text, and `Prelude` re-exports the formatting names without the class.

`nat-to-string :: Nat -> String` and `int-to-string :: Int -> String` are the decimal renderings. `char-string :: Char -> String` encodes one character, `nat-digit-count :: Nat -> Nat` counts the digits, `pow10 :: Nat -> Nat` raises ten to the argument, and `nat-digits-into` writes `k` digits into a buffer at a position. `show :: Show a => a -> String` is the class rendering.

`show` needs `import Text.Show`, because `Prelude` re-exports the concrete conversions and not the class. `nat-to-string` counts the digits with `nat-digit-count`, allocates that many bytes, and fills them from the end with `nat-digits-into`, so the cost is one division per digit.

`nat-to-string 1234` is `"1234"`, `int-to-string (-42)` is `"-42"`, and `char-string 'a'` is `"a"`. `int-to-string` renders the sign as its own byte and formats the magnitude as a `Nat`, so an `Int` past the `I64` range still formats correctly, and `show 'answer` is the `Show Sym` instance, which prints the name with its leading apostrophe. A one-character literal with a closing quote is a `Char`; the apostrophe without one is a `Sym`.

# The complete program

The program converts the same `Nat` value through each family, keeps the two conversions that can fail in `Option`, and checks the text and byte forms agree.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Text.Show

  price :: Nat
  price = 1234

  wide :: Int
  wide = nat-to-int price

  narrowed :: Option I64
  narrowed = checked-nat-to-i64 price

  damaged :: Option Nat
  damaged = checked-sub price 5000

  applied :: Option U8
  applied = checked-add-u8 200 100

  clamped :: U8
  clamped = saturating-add-u8 200 100

  rendered :: String
  rendered = nat-to-string price

  encoded :: Bytes
  encoded = string-to-bytes rendered

  decoded :: Option String
  decoded = string-from-bytes encoded

  label :: Sym
  label = intern rendered

  main :: Int
  main =
    if wide == 1234
      && clamped == 255
      && is-none applied
      && is-none damaged
      && from-option 0 narrowed == 1234
      && from-option "" decoded == rendered
      && int-to-string wide == rendered
      && sym-name label == rendered
      && string-length rendered == 4
      && bytes-len encoded == 4
      then 0
      else 1
```

Read it in source order. `price` is an exact, unbounded `Nat`; `wide` is the same value as an `Int`, which needs no evidence. `narrowed` keeps the `I64` failure in an `Option`, because 1234 fits but the conversion can be given a value that does not. `damaged` and `applied` exercise the two failing arithmetic forms: `price - 5000` would be negative and 200 plus 100 would leave `U8`, so both answer `None`, which the program asserts with `is-none`. `clamped` performs that addition under the policy returning 255. `rendered` is the decimal text, `encoded` its UTF-8 bytes, `decoded` the optional round trip, and `label` the interned name. The final `if` compares every form, so `int-to-string wide == rendered` checks that the two formatting paths agree.

# What the compiler enforces

- **No implicit machine overflow.** There is no `Add` instance for a machine type, so every machine operation is a named function that states its policy.
- **Every conversion states its precondition.** `nat-from-int x` needs `0 <= x`, `int-to-i64 x` needs both `I64` bounds, and `bytes-get b i` needs `i < bytes-len b`. A branch condition, a named `Dec` witness, or a refined argument discharges one.
- **A refined value needs its predicate.** `bytes-set b 0 300` is refused at the literal, because 300 is not in `0` through `255`.
- **Byte indices are `Nat`.** A length converts to `I64` only under `n <= 9223372036854775807`, and the length type states that bound.
- **`string-from-bytes` can fail.** Its result is `Option String`, so invalid UTF-8 cannot be read as text without an explicit branch.
- **`Sym` equality is identity.** Two equal names are the same cell, which is what makes the comparison a pointer test.

# Diagnostics you will meet

Each message is copied from `cinch check Main.cinch` on a file named `Main.cinch`.

Converting an `Int` to `Nat` without the sign fact reports the obligation:

```text
cinch: 3:14: the obligation `0 <= x` needs `Dec` evidence or a branch fact at this use (discharge it in the `Dec`/`decide` world: `case decide (0 <= x) of Yes w -> ...`, under a branch condition that implies it (`if ... then`, a guard, or the left operand of `&&`), or pass an argument whose refinement implies it): no evidence for `0 <= x` is in scope here (in Main.cinch)
```

Reading a byte without an index fact reports the bound it needs:

```text
cinch: 3:18: the obligation `0 < (Data.Bytes.bytes-len b)` needs `Dec` evidence or a branch fact at this use (discharge it in the `Dec`/`decide` world: `case decide (0 < (Data.Bytes.bytes-len b)) of Yes w -> ...`, under a branch condition that implies it (`if ... then`, a guard, or the left operand of `&&`), or pass an argument whose refinement implies it): no evidence for `0 < (Data.Bytes.bytes-len b)` is in scope here (in Main.cinch)
```

A byte value outside its range is refused at the literal, and the message shows the failed predicate:

```text
cinch: 3:31: integer literal 300 is not a InRange 0 255 Nat: its predicate `(0 <= 300 && 300 <= 255)` is false (a refined value needs its predicate to hold) (in Main.cinch)
```

A conversion whose argument is a known out-of-range numeral reports the false obligation rather than a type error: `cinch: 3:10: the obligation \`99999999999999999999 <= 9223372036854775807\` of this call is false: its arguments are known here and do not satisfy it (in Main.cinch)`. Treating the decoding result as text without the branch is `cinch: 3:15: type mismatch: expected Option String, got String (in Main.cinch)`.

# Exercises

### Exercise 1

Write `byte-at :: Bytes -> Nat -> Option Nat` that reads a byte and answers `None` when the index is past the end, and `first-letter :: Bytes -> Option Char` that answers the first character of the buffer as text.

<details><summary>Answer</summary>

Compare the index against the length, then read in the branch that proves it:

```cinch
  byte-at :: Bytes -> Nat -> Option Nat
  byte-at b i =
    if i < bytes-len b then Some (bytes-get b i) else None
```

`first-letter` decodes the buffer, which is where a buffer that is not UTF-8 is rejected, and then takes the character at index zero:

```cinch
  first-letter :: Bytes -> Option Char
  first-letter b = case string-from-bytes b of
    None -> None
    Some text -> string-char-at text 0
```

`string-char-at` counts characters, so index zero is the first scalar value whatever its encoded width.

</details>

### Exercise 2

Write `count-to :: Nat -> String` that returns the decimal text of a `U8`-sized value and refuses a value that does not fit, and `sum-bytes :: List U8 -> Option U8` that adds a list of bytes with the checked policy.

<details><summary>Answer</summary>

```cinch
  count-to :: Nat -> Option String
  count-to n = case checked-nat-to-i64 n of
    None -> None
    Some value -> case checked-i64-to-u8 value of
      None -> None
      Some byte -> Some (nat-to-string (nat-from-i64 (u8-to-i64 byte)))

  sum-bytes :: List U8 -> Option U8
  sum-bytes xs = case xs of
    Nil -> Some 0
    Cons first rest -> add-rest first rest

  add-rest :: U8 -> List U8 -> Option U8
  add-rest acc rest = case rest of
    Nil -> Some acc
    Cons y ys -> case checked-add-u8 acc y of
      None -> None
      Some sum -> add-rest sum ys
```

`count-to` narrows through `I64` because `Nat` has no direct `U8` conversion, and widens back with `u8-to-i64` so that `nat-from-i64` has non-negativity from the conversion's own result refinement. `sum-bytes` propagates the first overflow instead of wrapping.

</details>

[Chapter 8](08-modules.md) splits a program across files: export lists, imports, the sibling-file rule, and the diagnostics for a module or a name that cannot be found.
