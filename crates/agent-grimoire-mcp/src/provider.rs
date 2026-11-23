//! A verify-only `jsonwebtoken` crypto provider: RS256 (RSA PKCS#1 v1.5 with
//! SHA-256) and ES256 (ECDSA P-256 with SHA-256) on the `RustCrypto` crates.
//!
//! `jsonwebtoken` is used without a backend feature, so only these two
//! algorithms can ever verify, and nothing can sign
//! ([ADR-0007](https://github.com/jabbott-iii/Grimoire/blob/main/docs/adr/0007-mcp-server-authorization.md), on server-side MCP authorization).
//! RSA keys must be 2048 to 8192 bits (RFC 7518 §3.3).

use std::ops::RangeInclusive;

use jsonwebtoken::crypto::{CryptoProvider, JwtSigner, JwtVerifier, KeyUtils};
use jsonwebtoken::errors::{Error, ErrorKind};
use jsonwebtoken::signature::{self, Verifier};
use jsonwebtoken::{Algorithm, AlgorithmFamily, DecodingKey, DecodingKeyKind, EncodingKey};
use rsa::traits::SignatureScheme as _;
use sha2::Digest as _;

/// Accepted RSA modulus sizes, in bits.
const RSA_BITS: RangeInclusive<usize> = 2048..=8192;

fn rsa_key(key: &DecodingKey) -> Option<rsa::PublicKey> {
    let DecodingKeyKind::RsaModulusExponent { n, e } = key.kind() else {
        return None;
    };
    rsa::PublicKey::new_with_size_limits(
        rsa::BoxedUint::from_be_slice_vartime(n),
        rsa::BoxedUint::from_be_slice_vartime(e),
        RSA_BITS,
    )
    .ok()
}

/// Whether an RSA key is within the accepted sizes.
pub(crate) fn rsa_key_is_usable(key: &DecodingKey) -> bool {
    rsa_key(key).is_some()
}

struct Rs256(rsa::PublicKey);

impl Verifier<Vec<u8>> for Rs256 {
    fn verify(&self, msg: &[u8], sig: &Vec<u8>) -> Result<(), signature::Error> {
        let digest = sha2::Sha256::digest(msg);
        rsa::Pkcs1v15Sign::new::<sha2::Sha256>()
            .verify(&self.0, &digest, sig)
            .map_err(signature::Error::from_source)
    }
}

impl JwtVerifier for Rs256 {
    fn algorithm(&self) -> Algorithm {
        Algorithm::RS256
    }
}

struct Es256(p256::ecdsa::VerifyingKey);

impl Verifier<Vec<u8>> for Es256 {
    fn verify(&self, msg: &[u8], sig: &Vec<u8>) -> Result<(), signature::Error> {
        let sig = p256::ecdsa::Signature::from_slice(sig).map_err(signature::Error::from_source)?;
        self.0.verify(msg, &sig)
    }
}

impl JwtVerifier for Es256 {
    fn algorithm(&self) -> Algorithm {
        Algorithm::ES256
    }
}

/// Only RS256 with an RSA key and ES256 with an EC key; every other pairing is
/// refused (RFC 8725 §3.1).
#[allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "the signature is fixed by jsonwebtoken's CryptoProvider"
)]
fn verifier(alg: &Algorithm, key: &DecodingKey) -> Result<Box<dyn JwtVerifier>, Error> {
    match (alg, key.family()) {
        (Algorithm::RS256, AlgorithmFamily::Rsa) => {
            let key =
                rsa_key(key).ok_or_else(|| Error::from(ErrorKind::InvalidRsaKey(String::new())))?;
            Ok(Box::new(Rs256(key)))
        }
        (Algorithm::ES256, AlgorithmFamily::Ec) => {
            let key = p256::ecdsa::VerifyingKey::from_sec1_bytes(key.try_get_as_bytes()?)
                .map_err(|_| Error::from(ErrorKind::InvalidEcdsaKey))?;
            Ok(Box::new(Es256(key)))
        }
        _ => Err(ErrorKind::InvalidAlgorithm.into()),
    }
}

#[allow(
    clippy::trivially_copy_pass_by_ref,
    reason = "the signature is fixed by jsonwebtoken's CryptoProvider"
)]
fn no_signer(_: &Algorithm, _: &EncodingKey) -> Result<Box<dyn JwtSigner>, Error> {
    Err(ErrorKind::InvalidAlgorithm.into())
}

/// The provider, installed once per process.
pub(crate) static VERIFY_ONLY: CryptoProvider = CryptoProvider {
    signer_factory: no_signer,
    verifier_factory: verifier,
    key_utils: KeyUtils::new_unimplemented(),
};
