//! RFC 5869 test case 1 and independently calculated HMAC-SHA-256 derivations.
//! PBKDF2 vectors use Python hashlib.pbkdf2_hmac. The long output crosses
//! the one-byte block-counter boundary, and includes a partial final block.
mod common;

#[test]
fn key_derivations_match_rfc_and_independent_vectors() {
    let dir = common::project("key_derivation", &[
        ("Crypto.Kdf.cinch", include_str!("../prelude/Crypto/Kdf.cinch")),
        ("Data.Bytes.Build.cinch", include_str!("../prelude/Data/Bytes/Build.cinch")),
        ("Text.Digest.cinch", include_str!("../prelude/Text/Digest.cinch")),
        ("Main.cinch", include_str!("fixtures/kdf-consumer.cinch")),
    ]);
    assert_eq!(common::run(&dir, "Main.cinch"), 0);
    common::cleanup(&dir);
}
