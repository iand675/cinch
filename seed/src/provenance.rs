//! L1 SLSA provenance for package releases.
//!
//! The compiler can produce unsigned in-toto Statements in DSSE envelopes.
//! Unsigned L1 evidence is honest about being forgeable; a hosted builder or
//! a trusted signer must add the signature that raises verification to L2.

use crate::{Error, Result};
use serde_json::{json, Value};
use sha2::{Digest as _, Sha256};

pub const STATEMENT_TYPE: &str = "https://in-toto.io/Statement/v1";
pub const PREDICATE_TYPE: &str = "https://slsa.dev/provenance/v1";
pub const BUILD_TYPE: &str = "https://cinch.systems/buildtypes/cinch/v1";
pub const PAYLOAD_TYPE: &str = "application/vnd.in-toto+json";

/// Result of structural provenance verification.
pub struct Check {
    pub verified: bool,
    pub level: &'static str,
    pub verification: &'static str,
}

/// Build an unsigned L1 provenance envelope for one release.
pub fn envelope(package_id: &str, name: &str, version: &str) -> Result<Value> {
    let statement = json!({
        "_type": STATEMENT_TYPE,
        "subject": [{
            "name": name,
            "digest": {"sha256": package_id.strip_prefix("sha256:").unwrap_or(package_id)}
        }],
        "predicateType": PREDICATE_TYPE,
        "predicate": {
            "buildDefinition": {
                "buildType": BUILD_TYPE,
                "externalParameters": {
                    "package": package_id,
                    "profile": "source",
                    "stageMode": "refresh"
                },
                "resolvedDependencies": [{
                    "uri": format!("pkg:cinch/{name}@{version}"),
                    "digest": {"sha256": package_id.strip_prefix("sha256:").unwrap_or(package_id)}
                }]
            },
            "runDetails": {
                "builder": {"id": format!("local:cinch/{}", env!("CARGO_PKG_VERSION"))},
                "metadata": {},
                "byproducts": []
            }
        }
    });
    let payload = serde_json::to_vec(&statement)
        .map_err(|e| Error::plain(format!("provenance statement: {e}")))?;
    Ok(json!({
        "payloadType": PAYLOAD_TYPE,
        "payload": base64_encode(&payload),
        "signatures": []
    }))
}

/// Write one JSON Lines provenance file for a release and return its digest.
pub fn write_jsonl(
    path: &std::path::Path,
    package_id: &str,
    name: &str,
    version: &str,
) -> Result<String> {
    let envelope = envelope(package_id, name, version)?;
    write_envelope(path, &envelope)
}

/// Write an existing DSSE envelope as one JSON Lines record and return the
/// SHA-256 digest of the exact bytes written.
pub fn write_envelope(path: &std::path::Path, envelope: &Value) -> Result<String> {
    let line = serde_json::to_string(envelope)
        .map_err(|e| Error::plain(format!("provenance JSON: {e}")))?;
    let text = format!("{line}\n");
    std::fs::write(path, &text).map_err(|e| Error::plain(format!("{}: {e}", path.display())))?;
    let mut digest = sha2::Sha256::new();
    sha2::Digest::update(&mut digest, text.as_bytes());
    Ok(format!(
        "sha256:{}",
        digest
            .finalize()
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    ))
}

/// Verify the structural SLSA fields and subject binding. Signatures are not
/// trusted here: an unsigned envelope is L1, a signed envelope is still L1
/// until a configured root of trust accepts the signer.
pub fn verify(envelope: &Value, package_id: &str) -> Result<Check> {
    if envelope.get("payloadType").and_then(Value::as_str) != Some(PAYLOAD_TYPE) {
        return Err(Error::plain("provenance payloadType is not in-toto JSON"));
    }
    let payload = envelope
        .get("payload")
        .and_then(Value::as_str)
        .ok_or_else(|| Error::plain("provenance payload is missing"))?;
    let bytes = base64_decode(payload).map_err(Error::plain)?;
    let statement: Value = serde_json::from_slice(&bytes)
        .map_err(|e| Error::plain(format!("provenance payload: {e}")))?;
    if statement.get("_type").and_then(Value::as_str) != Some(STATEMENT_TYPE) {
        return Err(Error::plain("provenance statement has the wrong _type"));
    }
    if statement.get("predicateType").and_then(Value::as_str) != Some(PREDICATE_TYPE) {
        return Err(Error::plain(
            "provenance predicateType is not SLSA provenance v1",
        ));
    }
    let expected = package_id.strip_prefix("sha256:").unwrap_or(package_id);
    let subject = statement
        .get("subject")
        .and_then(Value::as_array)
        .and_then(|subjects| {
            subjects.iter().find_map(|subject| {
                subject
                    .get("digest")
                    .and_then(|digest| digest.get("sha256"))
                    .and_then(Value::as_str)
            })
        })
        .ok_or_else(|| Error::plain("provenance subject digest is missing"))?;
    if subject != expected {
        return Err(Error::plain(
            "provenance subject digest does not match the release identity",
        ));
    }
    let predicate = statement
        .get("predicate")
        .ok_or_else(|| Error::plain("provenance predicate is missing"))?;
    let build = predicate
        .get("buildDefinition")
        .ok_or_else(|| Error::plain("provenance buildDefinition is missing"))?;
    if build.get("buildType").and_then(Value::as_str) != Some(BUILD_TYPE) {
        return Err(Error::plain(
            "provenance buildType is not the Cinch build type",
        ));
    }
    let external = build
        .get("externalParameters")
        .ok_or_else(|| Error::plain("provenance externalParameters is missing"))?;
    if external
        .get("package")
        .and_then(Value::as_str)
        .map(|value| value.strip_prefix("sha256:").unwrap_or(value))
        != Some(expected)
    {
        return Err(Error::plain(
            "provenance externalParameters.package does not match the release identity",
        ));
    }
    if predicate
        .pointer("/runDetails/builder/id")
        .and_then(Value::as_str)
        .is_none()
    {
        return Err(Error::plain("provenance builder id is missing"));
    }
    let signatures = envelope
        .get("signatures")
        .and_then(Value::as_array)
        .ok_or_else(|| Error::plain("provenance signatures must be an array"))?;
    Ok(if signatures.is_empty() {
        Check {
            verified: false,
            level: "SLSA_BUILD_LEVEL_1",
            verification: "unsigned-l1",
        }
    } else {
        Check {
            verified: false,
            level: "SLSA_BUILD_LEVEL_1",
            verification: "untrusted-signature",
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsigned_provenance_binds_the_release_and_reports_l1() {
        let package = format!("sha256:{}", "a".repeat(64));
        let envelope = envelope(&package, "alice/library", "0.1.0").expect("envelope");
        let check = verify(&envelope, &package).expect("verify");
        assert!(!check.verified);
        assert_eq!(check.level, "SLSA_BUILD_LEVEL_1");
        assert_eq!(check.verification, "unsigned-l1");
        assert!(verify(&envelope, &format!("sha256:{}", "b".repeat(64))).is_err());
    }
}

fn base64_encode(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in bytes.chunks(3) {
        let b = [
            chunk[0],
            *chunk.get(1).unwrap_or(&0),
            *chunk.get(2).unwrap_or(&0),
        ];
        let n = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        out.push(TABLE[((n >> 18) & 63) as usize] as char);
        out.push(TABLE[((n >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            TABLE[((n >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[(n & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

fn base64_decode(text: &str) -> std::result::Result<Vec<u8>, String> {
    let mut out = Vec::with_capacity(text.len() * 3 / 4);
    let mut buffer = 0u32;
    let mut bits = 0u32;
    for byte in text.bytes() {
        if byte == b'=' || byte == b'\n' || byte == b'\r' {
            continue;
        }
        let value = match byte {
            b'A'..=b'Z' => (byte - b'A') as u32,
            b'a'..=b'z' => (byte - b'a' + 26) as u32,
            b'0'..=b'9' => (byte - b'0' + 52) as u32,
            b'+' => 62,
            b'/' => 63,
            _ => return Err("invalid base64 in provenance payload".into()),
        };
        buffer = (buffer << 6) | value;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((buffer >> bits) as u8);
            buffer &= (1 << bits) - 1;
        }
    }
    Ok(out)
}
