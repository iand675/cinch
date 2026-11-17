# Packed UUID parser candidate

This separate source snapshot changes only private parsing workers. It gathers
proved encoded byte ranges and constructs immutable words directly, removing
the compact32 and decoded16 temporary buffers. It retains the ordinary private
String-to-Bytes copy and makes no new backing access available. Existing
checked word reads hold their owners across raw loads. SIMD decoding reuses
the qualified Hex kernel algorithm under a private name, with both endian
variants. Invalid lanes are checked before value construction.

Public types, errors, accepted forms, receiver preservation, formatting,
append,comparison,hash,entropy and clock/state behavior remain unchanged.
The original candidate and all benchmark inputs stay plain and immutable.
This is static unqualified source until the gates in manifest.json pass;
no performance or production selection is claimed.

The complete standalone candidate matrix now passes exact330/d91:660 parser,
204append,370comparison,48recorded secure payload and9controlled-clock
observations, plus32,768 derived positional cases and real owner/entropy/
clock/shared-generator IO. All32 normal/poison8MiB-stack processes report
zero leaks. Every closed functional ELF/IR is durably archived. The positional
fixture's reserved `target` binder diagnostic is preserved; renaming that
fixture binder changed no library code or expectations. Parent's maintained
8-test/PPC gate and packed performance remain separate pending steps.
