//! CESRox - The Rust implementation of the CESR [specification]. See [this link] for a hosted view.<br>
//! Base64 in this documentation means the URL-safe [variant] of the Base64 encoding protocol.
//!
//! [specification]: https://github.com/WebOfTrust/ietf-cesr
//! [this link]: https://weboftrust.github.io/ietf-cesr/draft-ssmith-cesr.html
//! [variant]:  https://www.rfc-editor.org/rfc/rfc4648#section-5

/// Parses `Vec[u8]`s into raw types
pub mod derivation;

/// Listing of error types used across the crate
pub mod error;

/// Cryptographic keypair module for all supported key algorithms.
pub mod keys;

/// Parsing and raw type module for self certifying identifiers.
pub mod prefix;
