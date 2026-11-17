# Exact modular arithmetic foundation, static phase

`src/Data.Nat.Modular.cinch` is an isolated, unregistered candidate. No source
check, compiler invocation, reference execution, IR emission, object/link test,
runtime, model run or performance measurement is claimed. It does not change
the sealed suffix-index or Fields inputs. No shared inventory is edited.

Primary multiplication and power take `NonZero Nat`; checked variants refuse
zero with `ZeroModulus`. Their results are canonical residues. Modulus one
returns zero, including exponent zero. `nat-gcd` includes gcd(0,0)=0.
`nat-extended-gcd` returns an opaque divisor and signed coefficients satisfying
the exact Bezout identity. Both coefficients are zero for two zero inputs;
other valid coefficient choices need not match a reference's nonunique choice.
Inverse returns either a canonical residue or `NotCoprime` carrying the exact
gcd. A negative signed input is normalized before inversion. Signed powers
handle negative exponents by requiring that inverse first. Checked zero-modulus
refusal is an explicit API policy, not an imported panic or an unmodulated power.

The machine path first converts only values at most 2^64-1 using checked 32-bit
halves. It never routes an upper-half unsigned value through a signed I64 cast.
The private LLVM product zero-extends two U64 operands to i128, multiplies them
exactly, and takes `urem` only with `NonZero U64`. The largest product is less
than 2^128. Residue and small exponent loops remain unboxed. The arbitrary
exponent path halves a Nat until the remaining exponent fits U64.

The wide remainder can lower to a support symbol such as `__umodti3`; **neither
its actual lowering nor availability on any target has been checked**. Existing
wide multiply/shift kernels do not prove wide division availability. Actual
host, wasm, ARM and PPC object graphs, undefined symbols, matched runtime and
support-library linkage are mandatory before adopting this path. A portable
bounded-word division alternative may be required after that evidence.

The pure Nat fallback uses binary accumulation for multiplication and binary
squaring for power. Residues stay below the modulus; additions produce at most
one extra significant bit and reduce with one comparison/checked subtraction,
without another general division. It never constructs the complete product of two
modulus-sized residues. Every recursive call follows sequenced calculations
and is in tail position. Gcd and extended gcd use tail-position Euclidean
remainder/coefficient recurrences. Compiled bounded-stack behavior still needs
8 MiB program-stack stress under both frontends and optimization levels.

These are variable-time algorithms, with input-dependent iterations and
branches. No constant-time secret-key primitive is provided. Exact modular
arithmetic alone does not establish primality, a random-prime distribution,
Miller–Rabin/Lucas policy, a witness source, or a cryptographic Prime capability.
Those remain separate reviewed foundations.

The dependency audit reads the actual Nat/Int/Rational source and runtime
integer/limb implementations. Nat exposes exact add/mul/div/rem, comparison
and lossless Nat/Int conversion, but no public limb view, gcd, modular power or
inverse. Rational has a private Euclid implementation, not an exported gcd.
Runtime magnitude storage is private; this candidate does not borrow it or add
any runtime ABI. Existing signed division truncates toward zero, so it is not
silently mapped to a Euclidean quotient API.

`surface.json` records every actual cumulative `math/big` declaration and maps
only concrete existing operations or this explicitly unqualified candidate.
Signed/receiver/domain differences and all remaining declarations stay open.
This is not full package coverage. Big floats, roots, general integer bit APIs,
serialization, formatting and primality are not supplied by this increment.

`models.py` contains independent executable loop invariants and Python exact
arithmetic comparisons, **not yet run**. `oracle.go` invokes the actual SDK
integer operations. `generate.py` prepares deterministic fringe inputs and
will verify the SDK observations against independent multiplication, modular
power, gcd and inverse results after a resource grant. Zero-modulus records
are explicitly candidate policy and do not falsely claim an SDK refusal.

Qualification must include all small moduli, modulus one, zero exponents,
negative inverses, non-coprime gcds, 32/63/64/65/127/128-bit fringes, even and odd
moduli, full upper-half U64 residues, huge exponents, composite moduli and
4096-bit operands. Retain inputs/results/errors/Bezout coefficients past each
other's last use to test managed ownership. Use normal 32 MiB and poison
256 MiB heaps with 8 MiB stacks, compiled checked-RC API gates separately, and
both optimization levels/frontends. Refined NonZero inputs must refuse literal
zero and effect capabilities must not be fabricated by wrappers.

Public benchmarks should separate multiplication, repeated power with one
retained modulus, inverse/gcd, large exponent with word modulus, and fallback
large-modulus power. Compare word and Nat-only implementations on the same
inputs, plus the pinned reference. Cover 8/32/63/64/65/128/256/1024/4096-bit
moduli, sparse/dense exponents, and coprime/non-coprime inputs. Consume complete
results and retained input fingerprints outside the clock; keep owners alive
through the end clock. Record allocation diagnostics separately, inspect actual
boxing/call graphs and tail recursion, and retain regressions. No timing or
acceleration claim precedes functional preflights and an exclusive lease.
