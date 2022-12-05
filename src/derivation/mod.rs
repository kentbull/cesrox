pub mod attached_signature_code;
pub mod basic;
pub mod self_addressing;
pub mod self_signing;

/// Derivation codes are the type system of CESR supported data types. Each code corresponds to one and exactly one
/// entry in the CESR [master code table].<br>
/// Describes the length of both the derivation code as well as the derivation of the data type.
/// Also represents the code as a string value.
///
/// [master code table]: https://weboftrust.github.io/ietf-cesr/draft-ssmith-cesr.html#name-master-code-table
pub trait DerivationCode {
    fn code_len(&self) -> usize;
    fn derivative_b64_len(&self) -> usize;
    fn prefix_b64_len(&self) -> usize {
        self.code_len() + self.derivative_b64_len()
    }
    fn to_str(&self) -> String;
}
