# 10. Effects, capabilities, and a program that prints

An effect row says what a computation can do. A capability is a value granting access to a particular runtime. You need both pieces for an I/O operation: a type that admits its effect and a capability available at the call.

# Print a value

<!-- cinch:run exit=0 stdout="42" -->
```cinch
module Main where
  main :: Unit ->{IO r} Int
  main u = with (io = newIO ()) in
    let written = ioWrite (string-to-bytes (int-to-string 42))
    in 0
```

This entry point takes `Unit` and returns an exit status after performing I/O. The compiler supplies the entry argument. `newIO ()` creates a runtime capability, and `with (io = ...) in ...` supplies it lexically. `ioWrite` writes bytes, so `string-to-bytes` encodes the formatted text as UTF-8; it does not automatically add a newline. Use `"42\n"` to print a line.

`->{IO r}` is an effectful arrow. A pure arrow is written `->`. The region identity `r` ties the effect to a runtime. A helper normally quantifies that identity explicitly:

```text
greet :: forall r. using (io :: Io r) => String ->{IO r} Unit
```

The `using` clause requires a capability, and the row permits the effect. Omitting either is not a way to make the operation pure. A capability can be forwarded in another `with` clause or supplied at a call with `@{io = ...}`.

# Sequence steps with do

In the direct provider, `do` sequences computations in source order. A binding `x <- action` gives the next step the action's result. A `let x = expression` names a local result. The last expression supplies the block's result. Direct sequencing is the default; it does not wrap results in an `IO` datatype.

Cinch also supports `ResultSequencing e` and `OptionSequencing` providers. They use the same surface `do` syntax but select different `pure` and `bind` methods. A definition's provider is selected lexically. Container-polymorphic library code uses the `Category.*` abstractions instead. See [effects and sequencing](../reference/09-effects-and-capabilities.md) for complete provider rules.

# Describe every performed effect

An effectful helper cannot hide I/O behind a pure signature. Likewise, a function that may diverge must account for `Diverge`. Effect rows may have polymorphic tails, allowing a higher-order function to propagate its callback's effects. This is why you should read the callback arrow as well as the outer function arrow when choosing a traversal.

A row describes permission and possibility; it does not certify that an operation happened, or that it finished. An I/O capability does not establish termination, and a handler can introduce effects of its own.

# Exercise and answer

Change the program to print the total of `[3, 4, 5]` followed by a newline. Keep success as exit 0.

<details><summary>Answer</summary>

Calculate the total with a pure helper from chapter 5. Format it using `nat-to-string`, join `"\n"` with `string-concat`, encode the result with `string-to-bytes`, and pass it to `ioWrite` inside the capability scope. Keep `main`'s `IO r` arrow. The pure helper needs no I/O effect.

</details>
