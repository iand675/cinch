# Prime generation contract and blocked implementation

The concrete `Data.Prime.Candidate` proposal only shapes a candidate. It must not be exported or advertised as a prime generator. For valid width it copies input, preserves caller bytes, masks excess bits, forces the highest two bits and makes the value odd. Many results are composite. The two-bit case shapes to three; the one-high-bit byte fringe at widths 9, 17, etc. also forces the next byte's high bit.

An idiomatic future default could have this shape, once the missing foundations are maintained:

```cinch
-- | Generate an exact-width probable prime from actual system entropy.
-- The result is probabilistic, not a proof of primality. Invalid sizes are
-- named refusals; source and policy behavior are stated separately.
random-prime :: forall r. using (io :: Io r) => Nat ->{IO r} Result RandomPrimeError Nat
```

No such declaration/body is placed in a compilable proposed module today. A signed adapter can return a named invalid-size result for all values below two before conversion to Nat. The default must always use the actual secure source; a separately named explicit-source operation, if added, must not imply security for a deterministic callback. Do not copy a deprecated mutable environment switch into the library surface.

Required maintained dependencies and independent gates:

1. Arbitrary-precision modular exponentiation with independently checked reduction, exponent decomposition, zero/one and large-width behavior. An efficient implementation likely needs a maintained limb/reduction kernel; generic Nat arithmetic is a possible reference model, not a performance claim.
2. A reviewed probable-primality contract. The pinned reference uses screening plus Miller–Rabin and Lucas/Baillie–PSW. Its documented accuracy below 2^64 and probability claims cannot be replaced with a weaker unqualified routine. Carmichael numbers, strong pseudoprimes, squares, prime powers, boundary values and adversarial inputs need independent fixtures.
3. Explicit crypto-policy behavior, including any supported enforced-policy refusal. No equivalent maintained policy capability was found. A build that does not provide such policy must state that limit rather than claim certification or silently emulate it.
4. Actual-provider generation, exact bit length/top-two/parity checks, probabilistic behavior and ownership/leak checks. For arbitrary signed sizes, document which invalid inputs are representable and how they map to named errors.
5. Reader-error and advanced-capability behavior for any opt-in source API, including full-plus-error suppression, partial EOF, zero progress and invalid callback counts. The raw exact-read helper is a proposal for this dependency, not yet qualified.

The independent SDK generator separates `secure-default` from `prime-compat`. The former supplies a deliberately failing argument reader and records whether the actual default ignores it; it never replaces the internal secure source. The latter requires explicit deprecated compatibility mode and replays known prime candidates only as deterministic model fixtures. Optional probe consumption is recorded, not assumed. Policy-only and invalid-size precedence are captured through actual errors when those fixtures are later run under a supported policy mode.
