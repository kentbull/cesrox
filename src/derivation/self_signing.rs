use super::DerivationCode;
use crate::{error::Error, prefix::SelfSigningPrefix};
use core::str::FromStr;

/// Self Signing Derivations
///
/// A self signing prefix derivation outputs a signature as its derivative.
/// See section 2.3.5 of the [KERI whitepaper]
///
/// [KERI white paper]: https://github.com/SmithSamuelM/Papers/blob/master/whitepapers/KERI_WP_2.x.web.pdf
#[derive(Debug, PartialEq, Clone, Copy, Hash)]
pub enum SelfSigning {
    Ed25519Sha512,
    ECDSAsecp256k1Sha256,
    Ed448,
}

impl SelfSigning {
    pub fn derive(&self, sig: Vec<u8>) -> SelfSigningPrefix {
        SelfSigningPrefix::new(*self, sig)
    }
}

impl DerivationCode for SelfSigning {
    fn code_len(&self) -> usize {
        match self {
            Self::Ed25519Sha512 | Self::ECDSAsecp256k1Sha256 => 2,
            Self::Ed448 => 4,
        }
    }

    fn derivative_b64_len(&self) -> usize {
        match self {
            Self::Ed25519Sha512 | Self::ECDSAsecp256k1Sha256 => 86,
            Self::Ed448 => 152,
        }
    }

    fn to_str(&self) -> String {
        match self {
            Self::Ed25519Sha512 => "0B",
            Self::ECDSAsecp256k1Sha256 => "0C",
            Self::Ed448 => "1AAE",
        }
        .into()
    }
}

/// Maps self signing identifier data type strings to entries in the [master code table].
///
/// [master code table]: https://weboftrust.github.io/ietf-cesr/draft-ssmith-cesr.html#name-master-code-table
impl FromStr for SelfSigning {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s
            .get(..1)
            .ok_or_else(|| Error::DeserializeError("Empty prefix".into()))?
        {
            "0" => match &s[1..2] {
                "B" => Ok(Self::Ed25519Sha512),
                "C" => Ok(Self::ECDSAsecp256k1Sha256),
                _ => Err(Error::DeserializeError(
                    "Unknown signature type code".into(),
                )),
            },
            "1" => match &s[1..4] {
                "AAE" => Ok(Self::Ed448),
                _ => Err(Error::DeserializeError(
                    "Unknown signature type code".into(),
                )),
            },
            _ => Err(Error::DeserializeError(format!(
                "Unknown master code: {}",
                s
            ))),
        }
    }
}

#[cfg(test)]
mod self_signing_tests {
    use crate::derivation::self_signing::SelfSigning;
    use crate::prefix::Prefix;

    #[test]
    fn test_self_signing() {
        let der = SelfSigning::Ed25519Sha512.derive(vec![0; 64]);
        assert_eq!(der.to_str(), ["0B".to_string(), "A".repeat(86)].join(""));

        let der = SelfSigning::ECDSAsecp256k1Sha256.derive(vec![0; 64]);
        assert_eq!(der.to_str(), ["0C".to_string(), "A".repeat(86)].join(""));

        let der = SelfSigning::Ed448.derive(vec![0; 114]);
        assert_eq!(der.to_str(), ["1AAE".to_string(), "A".repeat(152)].join(""));
    }
}
