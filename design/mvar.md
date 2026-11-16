# Native MVar

`import Effect.Concurrent.MVar` provides a shared variable that is empty
or contains one value. It is implemented by a native runtime box with a
mutex and FIFO wait queues. Each blocked call parks its scheduler context,
so it works when one worker runs every task. It has no dependency on STM.

| Operation | Behavior |
| --- | --- |
| `mvar-new x`, `mvar-new-empty ()` | Create a fresh full or empty variable. |
| `mvar-take m` | Wait for a value, remove it, and return it. |
| `mvar-put m x` | Wait for space, then fill it. |
| `mvar-read m` | Wait for a value and return it without removing it. |
| `mvar-swap m x` | Wait for a value and replace it atomically, returning the old value. |
| `mvar-try-take`, `mvar-try-read` | Return `None` immediately when empty. |
| `mvar-try-put` | Return `False` immediately when full. |
| `mvar-is-empty` | Return a snapshot of emptiness. |
| `mvar-*-or-cancel` | Cancellable forms of take, put, read, and swap. |

All operations carry the `MVar` effect. Blocking operations additionally
carry `Diverge`. Cancellable operations require `Io r` evidence and carry
`Cancel r`; they return `None` on cancellation, or `False` for put.
Plain blocking operations deliberately ignore cooperative cancellation.

```cinch
module Main where
  import Effect.Concurrent.MVar

  main :: Unit ->{MVar, Diverge} Int
  main u = do
    slot <- mvar-new-empty ()
    _ <- mvar-put slot (42 :: Nat)
    value <- mvar-take slot
    pure (if value == 42 then 0 else 1)
```

## Handoff and cancellation

Blocked takers, including swaps, receive values in registration order.
Blocked putters receive space in registration order. The operation that
creates a value or space completes the oldest request while holding the
mutex, before making the result visible to another caller. A new call,
including a nonblocking one, cannot steal a reservation.

A put completes every reader already waiting for its value, including
readers that registered after a taker. Reads therefore wake together
without taking the value away from a consumer. Queued swaps supply their
replacement during the same handoff; they never expose an intermediate
empty slot.

A waiter publishes its request before releasing the mutex and parks in
a loop over its completion record. The scheduler's wake token covers a
wake before parking, and spurious wakes only recheck the record.
Cancellation removes a pending request under that same mutex. A completed
handoff takes precedence over cancellation, so a successful put cannot
silently lose its value. Cancelled writers and swaps release their
pending replacement values immediately.

FIFO begins when a request registers. There is no scheduling bound or
fairness promise for acquiring the mutex. Nonblocking calls can briefly
wait for that mutex, but never wait for the slot's state to change.
Queue insertion and normal handoff take constant time; broadcasting to
readers and removing a cancelled request take time proportional to the
affected queue.

## Representation and ownership

The public constructor is private and wraps the opaque `MVarCell a`
runtime type. A handle is `Copy`. Sharing or transferring a handle across
a task boundary requires its element type to be shareable, because
another copy of the handle may still exist.

Typed compiler intrinsics specialize on the element's representation.
Raw words, counted objects, and tagged mathematical integers retain
their usual representation; 128-bit values are boxed inside the cell and
unboxed on return. Status is separate from the value word, so zero is a
valid stored scalar or `Unit` value.

Every counted input is marked shared before publication, including inputs
to currently private variables. The generic reference-count mark walk
can consequently treat the changing native payload as opaque. Reads take
a reference under the mutex; takes transfer the existing reference.
The box's Rust destructor releases the slot and its queues. This trades
atomic counts on private stored values for a simple publication rule.

Native fibers and the OS-thread fallback use the same park interface.
The Wasm cooperative backend runs queued work to complete a wait and
reports a blocked operation if no runnable task can satisfy it, matching
that backend's existing suspension limits.

The runtime unit tests exercise FIFO reservations, reader broadcast,
swaps, withdrawal, parked threads, and ownership. Compiler integration
tests run the public API with one and four workers, both ordinary and
reference-count-debug builds, with leak checking and a timeout. They
also cover cancellation racing handoff, raw/wide/counted values, and
refusal of pure access or unshareable cross-task contents.
