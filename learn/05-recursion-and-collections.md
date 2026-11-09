# 5. Recursion, lists, and maps

A list is either empty or an element followed by a shorter list, so a function over a list is either a constant or a call on that shorter list. This chapter walks that shape from the explicit recursive definition to the library fold that replaces it, names the collection helpers the base library actually ships, shows why `Nat` subtraction asks for evidence, and ends with maps for the cases where a key matters and a position does not. Put the program under discussion in `Main.cinch` and use two commands:

```sh
cinch check Main.cinch
cinch run Main.cinch
```

`check` typechecks without linking, so it answers most questions in about a second; `run` builds a native executable and returns that program's exit status. Build the compiler and put it on `PATH` with [using the compiler](../reference/01-using-the-compiler.md).

**Prerequisites.** [Chapter 4](04-data-and-matching.md) for data declarations and `case`, and [chapter 3](03-functions.md) for lambdas and application. No import is needed unless a section says so.

# Take a list apart

`List` is declared in `Data.Core` and re-exported by `Data.List`. These four lines are the declaration as the source writes it, not a file to compile:

```cinch
data List a where
  Nil :: List a
  Cons :: a -> List a -> List a
```

A function over a list therefore has two equations to write, and `case` makes you supply the empty one:

<!-- cinch:run exit=0 -->
```cinch
module Main where
  total :: List Nat -> Nat
  total xs = case xs of
    Nil -> 0
    Cons x rest -> x + total rest

  main :: Int
  main = if total [3, 4, 5] == 12 then 0 else 1
```

`Nil` returns the identity for `+`. `Cons x rest` binds the head and the tail, adds `x` to the result of the same function on `rest`, and terminates because each call receives one constructor fewer. `[3, 4, 5]` is list-literal notation built through the `FromList` class, which the expected type `List Nat` fixes to `Cons 3 (Cons 4 (Cons 5 Nil))`. `main :: Int` returns the process exit status, so the `if` is how a checking program reports success.

# Carry an accumulator

The version above leaves one addition waiting at every level. A second argument carries the partial answer instead, and the recursive call becomes the last action. This is a fragment of the complete program later in the chapter:

```cinch
  walk :: List Nat -> Nat -> Nat
  walk rest acc = case rest of
    Nil -> acc
    Cons y ys -> walk ys (acc + y)

  total-acc :: List Nat -> Nat
  total-acc xs = walk xs 0
```

On an empty list `walk` returns the accumulator, which already holds the sum of every element removed so far; on a cons it adds the head before recursing.

A local recursive function needs `letrec`, because a plain `let` binding is not recursive. Its group admits a name used in its own definition only under a lambda, a memo body, or a `delay`, so `letrec sum-to :: Nat -> Nat -> Nat; sum-to = \n acc -> if n == 0 then acc else sum-to (n - 1) (acc + n) in sum-to 10 0` checks while `letrec x = x + 1 in x` does not.

# Replace the walk with a fold

`foldr` takes the two pieces you would have written by hand: the answer for `Nil` and the combination for `Cons`. Written out, `total-fold xs = foldr (\x acc -> x + acc) 0 xs` is the same sum as `total`, and the complete program below declares exactly that.

Two modules declare a `foldr`. The one in `Data.List` is the concrete list fold reachable through `Prelude`; the one in `Category.Foldable` is a class method over any container and needs an explicit import. The same split exists for `map`, whose class method lives in `Category.Functor`. The class module also carries the free combinators, each with a real name:

| Name | Signature as declared | Result |
| --- | --- | --- |
| `foldr`, `foldl` | `(a -> b -> b) -> b -> t a -> b` | fold from a seed |
| `to-list`, `null` | `t a -> List a`, `t a -> Bool` | contents, emptiness |
| `sum`, `product` | `using (fold :: Foldable t, sr :: Semiring a) => t a -> a` | `zero` and `one` on empty |
| `maximum`, `minimum`, `fold-left1`, `fold-right1` | `Option a` result | extreme or element-seeded fold |

Every partial form reports an empty container as `Option` rather than faulting. `Data.List` also carries the seeded concrete twin `sum-list :: using (add :: Add a) => a -> List a -> a`, so `sum-list 0 xs` sums a list without a `Semiring` constraint, and `sum xs` is the container-generic spelling.

Selecting one of these combinators on a list literal written inline currently trips the base library's own law check, with the message quoted below. Passing the list through a parameter, as every function here does, checks.

# The collection helpers that exist

| Name | Signature | Result |
| --- | --- | --- |
| `reverse`, `take`, `drop`, `split-at` | `Nat` bound first where present | shortened or reordered list |
| `nth`, `head`, `tail`, `last`, `init` | index or list, `Option` result | element or remainder |
| `map` | `(a -> b) -> List a -> List b` | one result per element |
| `partition`, `span`, `break` | predicate, then `List a -> (List a, List a)` | two lists |
| `zip`, `zip-with`, `unzip` | truncated at the shorter side | pairs |
| `find`, `find-index`, `elem-index`, `lookup` | search, `Option` result | witness or position |
| `sort`, `sort-by`, `group-by`, `nub` | ordering or predicate, then a list | ordered or grouped |
| `scanl`, `scanr` | `(b -> a -> b) -> b -> List a -> List b` | running results |
| `replicate`, `iterate`, `cycle`, `unfold`, `subsequences` | `Nat` bound first | finite generated list |
| `intersperse`, `intercalate` | separator, then list or list of lists | joined |

Three properties differ from other languages' list libraries. There is no `filter`: `partition` returns the accepted elements and the rejected ones as a pair, and you take the component you want. The generators take a `Nat` bound, because every list-producing walk in this language must end, so `iterate` is finite and `cycle` of the empty list stays empty. The partial projections return `Option`, never a fault.

`Data.Nat` exports `nat-add`, `nat-sub`, `nat-mul`, `nat-div` and `nat-rem`, whose divisor and result carry refinements, plus `nat-cmp`, `checked-div`, `checked-rem`, `checked-sub`, `saturating-sub`, `nat-from-int`, `nat-to-int`, `nat-from-i64`, `nat-to-i64`, `checked-nat-from-i64` and `checked-nat-to-i64`.

# Nat subtraction asks for evidence

`a - b` on `Nat` states `b <= a` as an obligation on its arguments, and the checker wants evidence at the use site. The `margin` function in the complete program below pays it with a branch condition: inside the `else` of `if used <= limit`, the fact is in scope. A `decide` expression supplies the same evidence when the `Yes` witness is bound to a name, so `case decide (used <= limit) of Yes evidence -> ...` checks while `Yes _` does not.

`checked-sub` and `saturating-sub` answer the obligation instead of proving it: `checked-sub limit used` is `None` exactly when `used > limit`, and `saturating-sub` clamps at zero. Choose from the meaning of the subtraction, not from convenience.

# Maps keyed by order

`Map k v` is an ordered binary search tree over `Ord k`, declared in `Data.Map` with `Tip` and `Bin`. A lookup answers `Option`, because a key can be absent.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  prices :: Map Nat Nat
  prices = {1 => 7, 2 => 12}

  main :: Int
  main = case map-lookup 2 prices of
    None -> 1
    Some price -> if price == 12 then 0 else 2
```

`{1 => 7, 2 => 12}` is a map literal built through `FromMap`, and the signature fixes both types. It is not a record: a record declares named fields, while this is a `Map` value with an ordering. `map-insert` replaces the value at an existing key, and `map-delete`, `map-keys`, `map-values`, `map-to-list`, `map-size` and `map-fold` complete the module.

# The complete program

The program repeats the recursion, the accumulator, the fold and the generic fold, then exercises the helpers and one `Nat` obligation.

<!-- cinch:run exit=0 -->
```cinch
module Main where
  import Category.Foldable

  total :: List Nat -> Nat
  total xs = case xs of
    Nil -> 0
    Cons x rest -> x + total rest

  walk :: List Nat -> Nat -> Nat
  walk rest acc = case rest of
    Nil -> acc
    Cons y ys -> walk ys (acc + y)

  total-acc :: List Nat -> Nat
  total-acc xs = walk xs 0

  total-fold :: List Nat -> Nat
  total-fold xs = foldr (\x acc -> x + acc) 0 xs

  total-generic :: List Nat -> Nat
  total-generic xs = sum xs

  doubled :: List Nat -> List Nat
  doubled xs = map (\x -> x * 2) xs

  top :: List Nat -> Option Nat
  top xs = maximum xs

  margin :: Nat -> Nat -> Option Nat
  margin limit used = if used <= limit then Some (limit - used) else None

  sample :: List Nat
  sample = [3, 4, 5]

  main :: Int
  main =
    if total sample == 12
      && total-acc sample == 12
      && total-fold sample == 12
      && total-generic sample == 12
      && sum-list 0 (doubled [1, 2]) == 6
      && sum-list 0 (case partition (\x -> 3 < x) sample of
        (yes, _) -> yes) == 9
      && sum-list 0 (sort [3, 1, 2]) == 6
      && list-length sample == 3
      then case top sample of
        None -> 1
        Some largest -> case margin largest 0 of
          None -> 2
          Some free -> case margin 3 4 of
            Some _ -> 3
            None -> if free == 5 then 0 else 4
      else 5
```

Read it in source order. `import Category.Foldable` brings the class `foldr`, `sum` and `maximum` into scope; without it, `sum` and `maximum` are unknown names. `total` is the first example, `walk` and `total-acc` the accumulator pair with its seed `0`, and `total-fold` the same sum as one `foldr` call. `total-generic` is the container-generic `sum`, which needs `Semiring Nat` for the `zero` of an empty list. `doubled` maps a lambda, the `partition` comparison takes the accepted component of the returned pair, `top` is `maximum` and answers `Option` because `Nil` has no greatest element, and `margin` pays the subtraction obligation in its `if` condition.

`main` is one `if` over six comparisons followed by nested `case` expressions. `sum-list 0` supplies the seed so derived sums compare without a `Semiring` constraint, and the exit status names the first failing stage: 5 for a comparison, 1 or 2 for an absent `maximum` or margin, 3 for a margin that should have failed, and 0 for success.

# What the compiler enforces

- **Every constructor is covered and every branch has the declared result type.** A `case` on `List` needs both branches, and `total :: List Nat -> Nat` makes each one a `Nat`.
- **`Nat` subtraction needs `b <= a`.** A branch condition, a named `Dec` witness, or a refined argument discharges it.
- **`letrec` needs a guard.** A name in its own group may be used only under a lambda, a memo body, or a `delay`.
- **A list literal needs a known element type.** `[3, 4, 5]` is resolved through `FromList`, and an undetermined type is refused.
- **Termination is not proved for ordinary definitions.** Structural recursion is what makes the definition terminate, and you have to mean it: a recursive call on an argument that does not shrink is accepted, because `Diverge` is not yet required on an ordinary arrow.

<span class="support-tag support-tag--planned">Coming soon</span> **required totality evidence.** The core language specification gives `Diverge` as the effect saying a computation may not return, and the design expects a definition that cannot be shown productive to declare it. Enforcement on executable definitions is deferred, so the arrow alone states no termination claim today; productivity of `codata` observations is checked, which [chapter 15](15-streams-and-producers.md) covers.

# Diagnostics you will meet

Each message is copied from `cinch check Main.cinch` on a file named `Main.cinch`. A `case` that forgets a constructor reports `non-exhaustive patterns: constructor Cons is not covered`, naming the constructor the match is missing. An eager binding group is refused at the recursive use rather than at the declaration, with `cinch: 4:12: unguarded recursion; wrap the use in \`delay\` or a lambda (the use of \`x\` closes an eager cycle in the binding group) (in Main.cinch)`.

Subtraction without evidence prints the whole obligation. Here it is for `difference limit used = limit - used`, wrapped to fit the page:

```text
cinch: 3:27: the obligation `used <= limit` needs `Dec` evidence or a branch fact at this use (discharge it in the `Dec`/`decide` world: `case decide (used <= limit) of Yes w -> ...`, under a branch condition that implies it (`if ... then`, a guard, or the left operand of `&&`), or pass an argument whose refinement implies it): no evidence for `used <= limit` is in scope here (in Main.cinch)
```

The law-check refusal for an inline literal names the instance and the predicate that does not survive:

```text
cinch: 5:11: instance `Data.List.FromList_List` breaks preservation law `NonEmpty.preserve-from-list` at sample values ($a0 = <adt 0 >): `from-list` produced `<adt 0 >`, which does not satisfy `NotNull xs` (refinement types design: a container's methods transfer exactly when they preserve the refinement)
```

That message carries no `(in FILE)` suffix, unlike the others. The workaround is a declaration of its own, `xs :: List Nat` with `xs = [3, 4, 5]`, so the literal is resolved before the combinator is selected.

# Exercises

### Exercise 1

Write `count :: List a -> Nat` returning the number of elements, and `all-over :: Nat -> List Nat -> Bool` returning whether every element is greater than a bound. `count` must not inspect its elements.

<details><summary>Answer</summary>

`count` needs a type variable with no constraint, because it never looks at an element. `all-over` returns `False` on the first element that fails the bound, which is a recursive call in the failing branch:

```cinch
  count :: List a -> Nat
  count xs = case xs of
    Nil -> 0
    Cons _ rest -> 1 + count rest

  all-over :: Nat -> List Nat -> Bool
  all-over bound xs = case xs of
    Nil -> True
    Cons y rest -> if bound < y then all-over bound rest else False
```

`list-length` already computes `count`, and `all (\y -> bound < y) xs` computes `all-over`.

</details>

### Exercise 2

Write `deliver :: Nat -> List Nat -> Option Nat` that treats each list element as an amount claimed against a budget, and returns the remaining budget or `None` when the claims exceed it. Use the accumulator shape and keep the subtraction total.

<details><summary>Answer</summary>

Carry the remaining budget as the accumulator, so the subtraction runs inside the branch that proves it:

```cinch
  deliver :: Nat -> List Nat -> Option Nat
  deliver budget claims = spend claims budget

  spend :: List Nat -> Nat -> Option Nat
  spend rest left = case rest of
    Nil -> Some left
    Cons amount later ->
      if amount <= left then spend later (left - amount) else None
```

The `if amount <= left` condition discharges the obligation on `left - amount`, and `checked-sub left amount` would compute the same number with the policy spread over two branches. Returning `None` keeps the failure in the result, which [chapter 6](06-options-results-and-validation.md) develops.

</details>

[Chapter 6](06-options-results-and-validation.md) replaces the exit status with values: `Option` for an absent answer and `Result` for a failure carrying an explanation, so a caller can react without reading a numeric code.
