//! Signing test tokens with the RFC 7515 example keys. Tokens are built at run
//! time, so the repository holds no token-shaped literal.

#![allow(dead_code, reason = "each test binary uses a different subset")]

use std::path::{Path, PathBuf};

use base64::Engine as _;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use serde_json::{Value, json};

pub(crate) const RESOURCE: &str = "https://docs.example.com/mcp";
pub(crate) const ISSUER: &str = "https://auth.example.com";

pub(crate) fn keys() -> Value {
    serde_json::from_str(include_str!("../fixtures/rfc7515-keys.json")).unwrap()
}

fn bytes(v: &Value) -> Vec<u8> {
    URL_SAFE_NO_PAD.decode(v.as_str().unwrap()).unwrap()
}

fn enc(v: &Value) -> String {
    URL_SAFE_NO_PAD.encode(v.to_string())
}

/// The public JWK for the RSA key, with `kid`.
pub(crate) fn rsa_jwk(kid: &str) -> Value {
    let k = &keys()["rsa"];
    json!({ "kty": "RSA", "kid": kid, "use": "sig", "n": k["n"], "e": k["e"] })
}

/// The public JWK for the P-256 key, with `kid`.
pub(crate) fn ec_jwk(kid: &str) -> Value {
    let k = &keys()["ec"];
    json!({ "kty": "EC", "kid": kid, "crv": "P-256", "x": k["x"], "y": k["y"] })
}

/// Writes `{"keys": [...]}` into `dir/jwks.json`.
pub(crate) fn write_jwks(dir: &Path, keys: &[Value]) -> PathBuf {
    let path = dir.join("jwks.json");
    std::fs::write(&path, json!({ "keys": keys }).to_string()).unwrap();
    path
}

pub(crate) fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

/// Claims for a token this server should accept.
pub(crate) fn good_claims() -> Value {
    json!({
        "iss": ISSUER,
        "aud": RESOURCE,
        "sub": "user-1",
        "client_id": "client-1",
        "exp": now() + 300,
        "iat": now(),
        "jti": "j1",
        "scope": "docs:read other",
    })
}

pub(crate) fn sign_es256(header: &Value, claims: &Value) -> String {
    use p256::ecdsa::signature::Signer as _;
    let key = p256::ecdsa::SigningKey::from_slice(&bytes(&keys()["ec"]["d"])).unwrap();
    let message = format!("{}.{}", enc(header), enc(claims));
    let signature: p256::ecdsa::Signature = key.sign(message.as_bytes());
    format!("{message}.{}", URL_SAFE_NO_PAD.encode(signature.to_bytes()))
}

pub(crate) fn sign_rs256(header: &Value, claims: &Value) -> String {
    use sha2::Digest as _;
    let k = keys()["rsa"].clone();
    let n = |name: &str| rsa::BoxedUint::from_be_slice_vartime(&bytes(&k[name]));
    let key =
        rsa::PrivateKey::from_components(n("n"), n("e"), n("d"), vec![n("p"), n("q")]).unwrap();
    let message = format!("{}.{}", enc(header), enc(claims));
    let digest = sha2::Sha256::digest(message.as_bytes());
    let signature = key
        .sign(rsa::Pkcs1v15Sign::new::<sha2::Sha256>(), &digest)
        .unwrap();
    format!("{message}.{}", URL_SAFE_NO_PAD.encode(signature))
}

/// An ES256 access token with these claims.
pub(crate) fn es256(claims: &Value) -> String {
    sign_es256(
        &json!({ "alg": "ES256", "typ": "at+jwt", "kid": "ec1" }),
        claims,
    )
}

/// An RS256 access token with these claims.
pub(crate) fn rs256(claims: &Value) -> String {
    sign_rs256(
        &json!({ "alg": "RS256", "typ": "at+jwt", "kid": "rsa1" }),
        claims,
    )
}
