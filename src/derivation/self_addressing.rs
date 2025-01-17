use super::DerivationCode;
use crate::{error::Error, prefix::SelfAddressingPrefix};
use blake2::{Blake2b, Digest, VarBlake2b, VarBlake2s};
use blake3;
use core::str::FromStr;
use sha2::{Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};

//     sha2::{Sha256, Sha512},
//     sha3::{Sha3_256, Sha3_512},
//     Digest,
// };

/// Self Addressing Derivations
///
/// Self-addressing is a digest/hash of some inception data (2.3.2)
///   Delegated Self-addressing uses the "dip" event data for the inception data (2.3.4)
#[derive(Debug, PartialEq, Clone, Hash)]
pub enum SelfAddressing {
    Blake3_256,
    Blake2B256(Vec<u8>),
    Blake2S256(Vec<u8>),
    SHA3_256,
    SHA2_256,
    Blake3_512,
    SHA3_512,
    Blake2B512,
    SHA2_512,
}

impl SelfAddressing {
    pub fn digest(&self, data: &[u8]) -> Vec<u8> {
        match self {
            Self::Blake3_256 => blake3_256_digest(data),
            Self::Blake2B256(key) => blake2b_256_digest(data, key),
            Self::Blake2S256(key) => blake2s_256_digest(data, key),
            Self::SHA3_256 => sha3_256_digest(data),
            Self::SHA2_256 => sha2_256_digest(data),
            Self::Blake3_512 => blake3_512_digest(data),
            Self::SHA3_512 => sha3_512_digest(data),
            Self::Blake2B512 => blake2b_512_digest(data),
            Self::SHA2_512 => sha2_512_digest(data),
        }
    }

    pub fn derive(&self, data: &[u8]) -> SelfAddressingPrefix {
        SelfAddressingPrefix::new(self.to_owned(), self.digest(data))
    }
}

impl DerivationCode for SelfAddressing {
    fn code_len(&self) -> usize {
        match self {
            Self::Blake3_256
            | Self::Blake2B256(_)
            | Self::Blake2S256(_)
            | Self::SHA3_256
            | Self::SHA2_256 => 1,
            Self::Blake3_512 | Self::SHA3_512 | Self::Blake2B512 | Self::SHA2_512 => 2,
        }
    }

    fn derivative_b64_len(&self) -> usize {
        match self {
            Self::Blake3_256
            | Self::Blake2B256(_)
            | Self::Blake2S256(_)
            | Self::SHA3_256
            | Self::SHA2_256 => 43,
            Self::Blake3_512 | Self::SHA3_512 | Self::Blake2B512 | Self::SHA2_512 => 86,
        }
    }

    fn to_str(&self) -> String {
        match self {
            Self::Blake3_256 => "E",
            Self::Blake2B256(_) => "F",
            Self::Blake2S256(_) => "G",
            Self::SHA3_256 => "H",
            Self::SHA2_256 => "I",
            Self::Blake3_512 => "0D",
            Self::SHA3_512 => "0E",
            Self::Blake2B512 => "0F",
            Self::SHA2_512 => "0G",
        }
        .into()
    }
}

impl FromStr for SelfAddressing {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .get(..1)
            .ok_or_else(|| Error::DeserializeError("Empty prefix".into()))?
        {
            "E" => Ok(Self::Blake3_256),
            "F" => Ok(Self::Blake2B256(vec![])),
            "G" => Ok(Self::Blake2S256(vec![])),
            "H" => Ok(Self::SHA3_256),
            "I" => Ok(Self::SHA2_256),
            "0" => match &s[1..2] {
                "D" => Ok(Self::Blake3_512),
                "E" => Ok(Self::SHA3_512),
                "F" => Ok(Self::Blake2B512),
                "G" => Ok(Self::SHA2_512),
                _ => Err(Error::DeserializeError("Unknown hash code".into())),
            },
            _ => Err(Error::DeserializeError(
                "Unknown hash algorithm code".into(),
            )),
        }
    }
}

fn blake3_256_digest(input: &[u8]) -> Vec<u8> {
    blake3::hash(input).as_bytes().to_vec()
}

fn blake2s_256_digest(input: &[u8], key: &[u8]) -> Vec<u8> {
    use blake2::digest::{Update, VariableOutput};
    let mut hasher = VarBlake2s::new_keyed(key, 256);
    hasher.update(input);
    hasher.finalize_boxed().to_vec()
}

// TODO it seems that blake2b is always defined as outputting 512 bits?
// TODO updated -> is this the one?
fn blake2b_256_digest(input: &[u8], key: &[u8]) -> Vec<u8> {
    use blake2::digest::{Update, VariableOutput};
    let mut hasher = VarBlake2b::new_keyed(key, 256);
    hasher.update(input);
    hasher.finalize_boxed().to_vec()
}

fn blake3_512_digest(input: &[u8]) -> Vec<u8> {
    let mut out = [0u8; 64];
    let mut h = blake3::Hasher::new();
    h.update(input);
    h.finalize_xof().fill(&mut out);
    out.to_vec()
}

fn blake2b_512_digest(input: &[u8]) -> Vec<u8> {
    let mut hasher = Blake2b::new();
    hasher.update(input);
    hasher.finalize().to_vec()
}

fn sha3_256_digest(input: &[u8]) -> Vec<u8> {
    let mut h = Sha3_256::new();
    h.update(input);
    h.finalize().to_vec()
}

fn sha2_256_digest(input: &[u8]) -> Vec<u8> {
    let mut h = Sha256::new();
    h.update(input);
    h.finalize().to_vec()
}

fn sha3_512_digest(input: &[u8]) -> Vec<u8> {
    let mut h = Sha3_512::new();
    h.update(input);
    h.finalize().to_vec()
}

fn sha2_512_digest(input: &[u8]) -> Vec<u8> {
    let mut h = Sha512::new();
    h.update(input);
    h.finalize().to_vec()
}

#[cfg(test)]
mod self_addressing_tests {
    use crate::derivation::self_addressing::SelfAddressing;
    use crate::prefix::Prefix;
    use base64::URL_SAFE;
    use std::str;

    #[test]
    fn test_self_addressing() {
        let der = SelfAddressing::Blake3_256.derive(b"abcdefghijklmnopqrstuvwxyz0123456789");
        assert_eq!(der.to_str(), "EsLkveIFUPvt38xhtgYYJRCCpAGO7WjjHVR37Pawv67E");

        let der = SelfAddressing::Blake3_512.derive(b"abcdefghijklmnopqrstuvwxyz0123456789");
        assert_eq!(der.to_str(), "0DsLkveIFUPvt38xhtgYYJRCCpAGO7WjjHVR37Pawv67GaNK7TEsvL8TnpL3E8EF7fR4KFgQaeMYFMM0mFNWmP-g");

        let der = SelfAddressing::SHA2_256.derive(b"abcdefghijklmnopqrstuvwxyz0123456789");
        assert_eq!(der.to_str(), "IAR_CmU450lEUFUD4emkJKz8iqGdn9yg95-7ts4l77fY");

        let der = SelfAddressing::SHA2_512.derive(b"abcdefghijklmnopqrstuvwxyz0123456789");
        assert_eq!(der.to_str(), "0GpZtJIWoOOiBEO3LGS9rlHUGzOtCKhqT7k2N43S-c04mYCewx5SWbO0VJOI0CZWE2K-cVSNQ5O-dtp-6wGDlHDA");

        let der = SelfAddressing::Blake2B512.derive(b"abcdefghijklmnopqrstuvwxyz0123456789");
        assert_eq!(der.to_str(), "0FefJm_XA2SP8c_tg8TZDS0tnho1iFCO15rIXDksOBFqkEpPFOu9r8hoEjBaFP_ewvUnZoOegECZ1_qEF5h9bcJQ");

        let der = SelfAddressing::SHA3_256.derive(b"abcdefghijklmnopqrstuvwxyz0123456789");
        assert_eq!(der.to_str(), "HAV1P0Jucuk5IyFE_LxP955z5dh52NpDhXmEnrqDJ8cU");

        let der = SelfAddressing::SHA3_512.derive(b"abcdefghijklmnopqrstuvwxyz0123456789");
        assert_eq!(der.to_str(), "0E59Emwi3GR06eDd87T1qgIq6of-KgJMIUsw2RtV0i3YSUDN4paOZtnqvOYEKt8MdX16f83bZnB-gcKby8aOIQcA");
    }
}
