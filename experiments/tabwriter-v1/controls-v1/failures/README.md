# Preserved adaptation fixture failures

These source-only attempts use the explicit qualified 330/d91 compiler/runtime tuple. Every attempt retains its original `Main.cinch`, exact command and tool/source identities in `report.json`, and original diagnostic. Full defining inputs remain plain in the owned build directories named by those reports. No attempt below emitted or executed a consumer, and the tabwriter library source is unchanged from checkpoint `3f5baacc`.

- `adaptation-source-v1`: unsupported fixture `\f` escape, before typing. The next fixture uses the same input byte 12 explicitly.
- `adaptation-source-v2`: multiline local conditional layout inserted a separator before `else`, before typing. Named expected-offer lists make selection explicit in the next fixture.
- `adaptation-source-v3`: the empty-prefix branch inferred its zero-length refinement for a conditional with longer byte branches. The prepared next fixture states the ordinary `Bytes` carrier explicitly; no library bounds or proof rule is changed.

All expected byte sequences, error families, signed reports, consumed checkpoints and panic-versus-return distinctions are retained. These failures are not successes, semantic refusals, or runtime observations. Current prepared controls remain unqualified until their separate source and runtime receipts pass.
