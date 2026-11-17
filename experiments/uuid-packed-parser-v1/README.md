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
