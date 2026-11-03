# 16. Delay work and memoize deliberately

Strict evaluation is the default. Use a thunk to postpone a computation, or a memo binding to share a delayed result within a controlled lifetime. They serve different purposes and have different usage rules.

# A thunk delays a function body

<!-- cinch:run exit=0 -->
```cinch
module Main where
  main :: Int
  main =
    let pending = delay (\() -> 20 + 22)
    in if force pending == 42 then 0 else 1
```

`delay` takes a function over `Unit`. Constructing the thunk does not run the function body. `force` executes it. The thunk's effects appear in its type and must be admitted when it is forced. Delaying I/O is not a way to erase I/O from a signature.

Ordinary thunks are affine by default. Forcing one consumes it, so a second force is rejected unless the type explicitly supports the appropriate repeated-use discipline. A duplicable `%many` thunk recomputes; it is not automatically a shared cache.

# Memo bindings share a result within a scope

A binding written `~answer = expression` creates a memoized result. Its initializer runs on first demand, and later demands use the cached result. A block owns a memo group, and that group's values cannot escape in a way that outlives the storage. In a `where` block the group's lifetime is per call, rather than a process-global cache.

Use `demand` to obtain the memo result's borrow, and use `copyOut` when you need an owned value and structural `Copy` evidence exists. Requiring a copy is important for generic code: a memo cache cannot make a linear resource duplicable.

The checker tracks an `Eval r` effect for a memo region and prevents reentrant demand using row constraints. The runtime has a black-hole defense if an invalid reentry reaches it. An eager cyclic value does not become useful simply because it is spelled with `~`.

# Match the tool to the lifetime

A one-shot delayed expensive calculation fits an affine thunk. Reusing a pure calculation several times within one invocation fits a memo binding. An unbounded service cache needs an explicit storage and eviction design; a local memo group is not that design. A productive stream tail uses `Later`, whose guarded meaning is different again.

**Open design question — irrefutable argument patterns.** Cinch's `~name = ...` is a memo binding. Haskell-style `~pattern` function arguments are not currently supported. The specification leaves this as an open decision, so it is not a promised addition. Keep it distinct from today's memo syntax.

# Exercise and answer

If forcing a `%many` thunk twice repeats an I/O operation, should you replace it with a memo binding without changing the surrounding explanation?

<details><summary>Answer</summary>

No. Recomputing twice and running once then reusing a result are observably different when effects are present. State the intended operation count, lifetime, and effects, then choose the construct. An optimization cannot silently change that behaviour.

</details>
