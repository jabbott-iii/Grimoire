//! OAuth 2.1 resource-server checks for Streamable HTTP (ADR-0007).
//!
//! An MCP server that requires authorization is an OAuth resource server
//! (`MCP-SPEC`). This module:
//! - serves RFC 9728 Protected Resource Metadata;
//! - takes the access token from the `Authorization: Bearer` header only,
//!   never from the query string;
//! - validates it as an RFC 9068 JWT access token, against keys in a JWKS file
//!   the operator supplies, with no outbound network: `typ`, signature
//!   (RS256 or ES256 only, the key bound to one algorithm), `iss`, `aud`,
//!   `exp` and `nbf`, and the required scopes;
//! - answers failures with RFC 6750 challenges that carry `resource_metadata`.
//!
//! Tokens are never logged, stored or passed on.

use std::path::PathBuf;
use std::sync::{Mutex, Once};
use std::time::{Duration, Instant, SystemTime};

use http::{HeaderMap, HeaderValue, StatusCode, header};
use jsonwebtoken::jwk::{
    AlgorithmParameters, EllipticCurve, Jwk, JwkSet, KeyAlgorithm, PublicKeyUse,
};
use jsonwebtoken::{Algorithm, AlgorithmFamily, DecodingKey, Validation};
use serde_json::Value;

use crate::provider;
use crate::url::Endpoint;

/// How often the JWKS file is checked for changes.
const RELOAD_INTERVAL: Duration = Duration::from_secs(5);
/// The largest JWKS file read.
const MAX_JWKS_BYTES: u64 = 1024 * 1024;
/// The largest access token accepted, in bytes.
const MAX_TOKEN_BYTES: usize = 16 * 1024;
/// The most clock skew tolerated for `exp` and `nbf`.
pub const MAX_LEEWAY_SECS: u64 = 60;
/// The RFC 9728 well-known path segment.
const WELL_KNOWN: &str = "/.well-known/oauth-protected-resource";

/// How a server checks access tokens.
#[derive(Debug, Clone)]
pub struct AuthConfig {
    /// The canonical URI of the MCP endpoint, e.g. `https://docs.example.com/mcp`.
    /// It is the resource in the metadata and the default token audience.
    pub resource: String,
    /// The authorization server's issuer identifier; tokens' `iss` must equal it.
    pub issuer: String,
    /// The JWKS file with the issuer's signing keys.
    pub jwks: PathBuf,
    /// More accepted `aud` values, besides `resource`.
    pub audiences: Vec<String>,
    /// Scopes every request needs.
    pub required_scopes: Vec<String>,
    /// A human-readable name for the metadata document.
    pub resource_name: Option<String>,
    /// Also accept `typ: JWT`. Off by default: RFC 9068 requires `at+jwt`,
    /// and the check keeps ID tokens out. Some authorization servers issue
    /// access tokens typed `JWT`.
    pub allow_jwt_typ: bool,
    /// Clock-skew tolerance in seconds, at most [`MAX_LEEWAY_SECS`].
    pub leeway_secs: u64,
}

/// A configuration or key-file problem found at startup.
#[derive(Debug)]
pub struct AuthError(pub String);

impl std::fmt::Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for AuthError {}

/// What a validated token says about the caller. Inserted into the request's
/// extensions, so tools can read it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Caller {
    /// The token's `sub`, when present.
    pub subject: Option<String>,
    /// The token's `client_id`, when present.
    pub client_id: Option<String>,
    /// The token's scopes.
    pub scopes: Vec<String>,
}

/// Why a request was refused, as an RFC 6750 challenge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Challenge {
    /// No credentials: 401 without an error code.
    Missing,
    /// Malformed credentials, or more than one: 400 `invalid_request`.
    InvalidRequest(&'static str),
    /// A token that was checked and failed: 401 `invalid_token`.
    InvalidToken(&'static str),
    /// A valid token without the scopes the request needs: 403 `insufficient_scope`.
    InsufficientScope,
}

/// One usable verification key.
struct SigningKey {
    kid: Option<String>,
    algorithm: Algorithm,
    key: DecodingKey,
}

struct KeyState {
    keys: Vec<SigningKey>,
    modified: Option<SystemTime>,
    checked: Instant,
}

/// Checks access tokens for one MCP endpoint.
pub struct Authenticator {
    config: AuthConfig,
    resource: Endpoint,
    audiences: Vec<String>,
    state: Mutex<KeyState>,
}

impl std::fmt::Debug for Authenticator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Authenticator")
            .field("resource", &self.config.resource)
            .field("issuer", &self.config.issuer)
            .finish_non_exhaustive()
    }
}

fn install_provider() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        // Fails only if a provider is already installed, which this crate
        // never does elsewhere; `check` then still uses the installed one.
        let _ = provider::VERIFY_ONLY.install_default();
    });
}

fn modified(path: &std::path::Path) -> Option<SystemTime> {
    std::fs::metadata(path).and_then(|m| m.modified()).ok()
}

/// Reads and filters the JWKS file. Keys that are not usable for RS256 (RSA,
/// 2048 bits or more) or ES256 (P-256) signatures are skipped.
/// Reads the JWKS file. `Ok(None)` when the file does not exist; otherwise
/// the usable keys, which may be none.
fn read_key_set(path: &std::path::Path) -> Result<Option<Vec<SigningKey>>, AuthError> {
    let shown = path.display();
    let meta = match std::fs::metadata(path) {
        Ok(meta) => meta,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(AuthError(format!("cannot read the JWKS file {shown}: {e}"))),
    };
    if !meta.is_file() || meta.len() > MAX_JWKS_BYTES {
        return Err(AuthError(format!(
            "the JWKS file {shown} must be a regular file of at most {MAX_JWKS_BYTES} bytes"
        )));
    }
    let text = std::fs::read_to_string(path)
        .map_err(|e| AuthError(format!("cannot read the JWKS file {shown}: {e}")))?;
    let set: JwkSet = serde_json::from_str(&text).map_err(|e| {
        AuthError(format!(
            "the JWKS file {shown} is not a JSON Web Key Set: {e}"
        ))
    })?;
    Ok(Some(set.keys.iter().filter_map(usable).collect()))
}

/// Loads the keys at startup, which needs at least one usable key.
fn load_keys(path: &std::path::Path) -> Result<Vec<SigningKey>, AuthError> {
    let shown = path.display();
    match read_key_set(path)? {
        None => Err(AuthError(format!("the JWKS file {shown} does not exist"))),
        Some(keys) if keys.is_empty() => Err(AuthError(format!(
            "the JWKS file {shown} has no usable signing key (RSA of 2048 bits or more for RS256, or P-256 for ES256)"
        ))),
        Some(keys) => Ok(keys),
    }
}

fn usable(jwk: &Jwk) -> Option<SigningKey> {
    if matches!(jwk.common.public_key_use, Some(ref u) if *u != PublicKeyUse::Signature) {
        return None;
    }
    let declared = jwk.common.key_algorithm;
    let algorithm = match &jwk.algorithm {
        AlgorithmParameters::RSA(_) => Algorithm::RS256,
        AlgorithmParameters::EllipticCurve(ec) if ec.curve == EllipticCurve::P256 => {
            Algorithm::ES256
        }
        _ => return None,
    };
    let expected = match algorithm {
        Algorithm::RS256 => KeyAlgorithm::RS256,
        _ => KeyAlgorithm::ES256,
    };
    // RFC 8725: each key is used with exactly one algorithm.
    if declared.is_some_and(|d| d != expected) {
        return None;
    }
    let key = DecodingKey::from_jwk(jwk).ok()?;
    if algorithm == Algorithm::RS256 && !provider::rsa_key_is_usable(&key) {
        return None;
    }
    Some(SigningKey {
        kid: jwk.common.key_id.clone(),
        algorithm,
        key,
    })
}

/// Splits `Authorization: Bearer <token>`; the scheme is case-insensitive.
fn bearer(headers: &HeaderMap) -> Result<&str, Challenge> {
    let mut values = headers.get_all(header::AUTHORIZATION).iter();
    let Some(value) = values.next() else {
        return Err(Challenge::Missing);
    };
    if values.next().is_some() {
        return Err(Challenge::InvalidRequest(
            "more than one Authorization header",
        ));
    }
    let value = value
        .to_str()
        .map_err(|_| Challenge::InvalidRequest("the Authorization header is not ASCII"))?;
    let (scheme, token) = value.split_once(' ').unwrap_or((value, ""));
    // Another scheme (Basic, DPoP, ...) is a request without Bearer credentials:
    // a plain challenge, with no error code (RFC 6750 §3.1).
    if !scheme.eq_ignore_ascii_case("bearer") {
        return Err(Challenge::Missing);
    }
    let token = token.trim_start_matches(' ');
    // RFC 6750 b64token: 1*( ALPHA / DIGIT / "-" / "." / "_" / "~" / "+" / "/" ) *"="
    let body = token.trim_end_matches('=');
    let ok = !body.is_empty()
        && token.len() <= MAX_TOKEN_BYTES
        && body
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-._~+/".contains(&b));
    if ok {
        Ok(token)
    } else {
        Err(Challenge::InvalidRequest("malformed bearer token"))
    }
}

fn scopes(claims: &Value) -> Vec<String> {
    // `scope` is a space-separated string (RFC 9068); some servers use `scp`,
    // as a string or an array.
    let from = |v: &Value| -> Vec<String> {
        match v {
            Value::String(s) => s
                .split(' ')
                .filter(|x| !x.is_empty())
                .map(str::to_owned)
                .collect(),
            Value::Array(items) => items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_owned)
                .collect(),
            _ => Vec::new(),
        }
    };
    claims
        .get("scope")
        .or_else(|| claims.get("scp"))
        .map(from)
        .unwrap_or_default()
}

/// Characters RFC 6750 allows in a quoted auth-param value we generate.
fn quoted(value: &str) -> String {
    value
        .chars()
        .filter(|c| matches!(*c, ' ' | '!' | '#'..='[' | ']'..='~'))
        .collect()
}

impl Authenticator {
    /// Checks the configuration and loads the keys.
    ///
    /// # Errors
    ///
    /// Fails for an invalid resource URI, a leeway over [`MAX_LEEWAY_SECS`], or a
    /// JWKS file that cannot be read or has no usable key.
    pub fn new(config: AuthConfig) -> Result<Self, AuthError> {
        install_provider();
        let resource = Endpoint::parse(&config.resource)
            .map_err(|e| AuthError(format!("--auth-resource {e}")))?;
        if config.issuer.trim().is_empty() {
            return Err(AuthError("the issuer must not be empty".to_owned()));
        }
        if config.leeway_secs > MAX_LEEWAY_SECS {
            return Err(AuthError(format!(
                "the clock-skew leeway may be at most {MAX_LEEWAY_SECS} seconds"
            )));
        }
        if let Some(bad) = config.required_scopes.iter().find(|s| {
            s.is_empty()
                || s.contains(|c: char| c == ' ' || c == '"' || c == '\\' || !c.is_ascii_graphic())
        }) {
            return Err(AuthError(format!("`{bad}` is not a valid scope")));
        }
        let keys = load_keys(&config.jwks)?;
        // The resource is always an accepted audience; configured ones are extra.
        let mut audiences = vec![config.resource.clone()];
        for audience in &config.audiences {
            if !audiences.contains(audience) {
                audiences.push(audience.clone());
            }
        }
        Ok(Self {
            state: Mutex::new(KeyState {
                keys,
                modified: modified(&config.jwks),
                checked: Instant::now(),
            }),
            resource,
            audiences,
            config,
        })
    }

    /// The configuration.
    #[must_use]
    pub fn config(&self) -> &AuthConfig {
        &self.config
    }

    /// The protected endpoint.
    pub(crate) fn endpoint(&self) -> &Endpoint {
        &self.resource
    }

    /// The path of the metadata document: the well-known suffix inserted before
    /// the resource's path (RFC 9728 §3.1).
    #[must_use]
    pub fn metadata_path(&self) -> String {
        let path = self.resource.path.trim_end_matches('/');
        format!("{WELL_KNOWN}{path}")
    }

    /// The absolute URL of the metadata document.
    #[must_use]
    pub fn metadata_url(&self) -> String {
        format!("{}{}", self.resource.origin(), self.metadata_path())
    }

    /// Whether `path` is where the metadata is served: the path-inserted URL,
    /// and the root URL, which some clients try first.
    pub(crate) fn is_metadata_path(&self, path: &str) -> bool {
        path == self.metadata_path() || path == WELL_KNOWN
    }

    /// The RFC 9728 metadata document.
    #[must_use]
    pub fn metadata(&self) -> Value {
        let mut doc = serde_json::Map::new();
        doc.insert(
            "resource".into(),
            Value::String(self.config.resource.clone()),
        );
        doc.insert(
            "authorization_servers".into(),
            Value::Array(vec![Value::String(self.config.issuer.clone())]),
        );
        if !self.config.required_scopes.is_empty() {
            doc.insert(
                "scopes_supported".into(),
                Value::Array(
                    self.config
                        .required_scopes
                        .iter()
                        .cloned()
                        .map(Value::String)
                        .collect(),
                ),
            );
        }
        doc.insert(
            "bearer_methods_supported".into(),
            serde_json::json!(["header"]),
        );
        if let Some(name) = &self.config.resource_name {
            doc.insert("resource_name".into(), Value::String(name.clone()));
        }
        Value::Object(doc)
    }

    /// Reloads the keys if the file changed, at most every [`RELOAD_INTERVAL`].
    ///
    /// A well-formed key set replaces the keys, even when it has no usable key:
    /// removing keys revokes them. A deleted file revokes every key. Only a file
    /// that cannot be read or parsed (for example one being rewritten in place)
    /// keeps the previous keys.
    fn refresh(&self, state: &mut KeyState) {
        if state.checked.elapsed() < RELOAD_INTERVAL {
            return;
        }
        state.checked = Instant::now();
        let now = modified(&self.config.jwks);
        if now == state.modified {
            return;
        }
        match read_key_set(&self.config.jwks) {
            Ok(Some(keys)) => {
                if keys.is_empty() {
                    crate::log(
                        "the JWKS file has no usable signing key: every token is refused until it has one",
                    );
                }
                state.keys = keys;
                state.modified = now;
            }
            Ok(None) => {
                crate::log("the JWKS file was removed: every token is refused until it is back");
                state.keys = Vec::new();
                state.modified = now;
            }
            Err(e) => crate::log(&format!("keeping the previous signing keys: {e}")),
        }
    }

    /// Checks a request's credentials.
    ///
    /// # Errors
    ///
    /// Returns the challenge to answer with.
    pub fn check(&self, headers: &HeaderMap) -> Result<Caller, Challenge> {
        let token = bearer(headers)?;
        let header = jsonwebtoken::decode_header(token)
            .map_err(|_| Challenge::InvalidToken("malformed token"))?;
        let typ_ok = header.typ.as_deref().is_some_and(|t| {
            t.eq_ignore_ascii_case("at+jwt")
                || t.eq_ignore_ascii_case("application/at+jwt")
                || (self.config.allow_jwt_typ && t.eq_ignore_ascii_case("JWT"))
        });
        if !typ_ok {
            return Err(Challenge::InvalidToken(
                "not an access token (typ must be at+jwt)",
            ));
        }
        if !matches!(header.alg, Algorithm::RS256 | Algorithm::ES256) {
            return Err(Challenge::InvalidToken("unsupported signature algorithm"));
        }
        let key = {
            let mut state = self
                .state
                .lock()
                .map_err(|_| Challenge::InvalidToken("signing keys unavailable"))?;
            self.refresh(&mut state);
            let candidates: Vec<&SigningKey> = state
                .keys
                .iter()
                .filter(|k| k.algorithm == header.alg)
                .filter(|k| match &header.kid {
                    Some(kid) => k.kid.as_deref() == Some(kid.as_str()),
                    None => true,
                })
                .collect();
            // Without a `kid`, only an unambiguous key is used.
            match candidates.as_slice() {
                [one] => one.key.clone(),
                _ => return Err(Challenge::InvalidToken("no matching signing key")),
            }
        };
        if key.family() != AlgorithmFamily::Rsa && header.alg == Algorithm::RS256 {
            return Err(Challenge::InvalidToken("no matching signing key"));
        }
        let mut validation = Validation::new(header.alg);
        validation.algorithms = vec![header.alg];
        validation.set_issuer(&[self.config.issuer.as_str()]);
        validation.set_audience(&self.audiences);
        validation.set_required_spec_claims(&["exp", "iss", "aud"]);
        validation.validate_nbf = true;
        validation.leeway = self.config.leeway_secs;
        let data = jsonwebtoken::decode::<Value>(token, &key, &validation).map_err(|e| {
            use jsonwebtoken::errors::ErrorKind as K;
            Challenge::InvalidToken(match e.kind() {
                K::ExpiredSignature => "the token has expired",
                K::ImmatureSignature => "the token is not valid yet",
                K::InvalidAudience => "the token was not issued for this server",
                K::InvalidIssuer => "the token was issued by another authorization server",
                K::MissingRequiredClaim(_) => "the token lacks a required claim",
                K::InvalidSignature => "invalid signature",
                _ => "invalid token",
            })
        })?;
        // `iss` must be the issuer string itself: jsonwebtoken also accepts an
        // array that merely contains it.
        if data.claims.get("iss").and_then(Value::as_str) != Some(self.config.issuer.as_str()) {
            return Err(Challenge::InvalidToken(
                "the token was issued by another authorization server",
            ));
        }
        let granted = scopes(&data.claims);
        if !self
            .config
            .required_scopes
            .iter()
            .all(|s| granted.contains(s))
        {
            return Err(Challenge::InsufficientScope);
        }
        Ok(Caller {
            subject: data
                .claims
                .get("sub")
                .and_then(Value::as_str)
                .map(str::to_owned),
            client_id: data
                .claims
                .get("client_id")
                .and_then(Value::as_str)
                .map(str::to_owned),
            scopes: granted,
        })
    }

    /// The status and `WWW-Authenticate` value for a challenge (RFC 6750 §3).
    #[must_use]
    pub fn challenge(&self, challenge: &Challenge) -> (StatusCode, HeaderValue) {
        let mut params = vec![format!(
            "resource_metadata=\"{}\"",
            quoted(&self.metadata_url())
        )];
        if !self.config.required_scopes.is_empty() {
            params.push(format!(
                "scope=\"{}\"",
                quoted(&self.config.required_scopes.join(" "))
            ));
        }
        let status = match challenge {
            Challenge::Missing => StatusCode::UNAUTHORIZED,
            Challenge::InvalidRequest(why) => {
                params.push("error=\"invalid_request\"".to_owned());
                params.push(format!("error_description=\"{}\"", quoted(why)));
                StatusCode::BAD_REQUEST
            }
            Challenge::InvalidToken(why) => {
                params.push("error=\"invalid_token\"".to_owned());
                params.push(format!("error_description=\"{}\"", quoted(why)));
                StatusCode::UNAUTHORIZED
            }
            Challenge::InsufficientScope => {
                params.push("error=\"insufficient_scope\"".to_owned());
                StatusCode::FORBIDDEN
            }
        };
        let value = format!("Bearer {}", params.join(", "));
        (
            status,
            HeaderValue::from_str(&value).unwrap_or_else(|_| HeaderValue::from_static("Bearer")),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bearer_parsing() {
        let mut h = HeaderMap::new();
        assert_eq!(bearer(&h), Err(Challenge::Missing));
        h.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer abc.def-_~+/=="),
        );
        assert_eq!(bearer(&h), Ok("abc.def-_~+/=="));
        h.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("bearer   abc"),
        );
        assert_eq!(bearer(&h), Ok("abc"));
        h.insert(header::AUTHORIZATION, HeaderValue::from_static("Basic abc"));
        assert_eq!(
            bearer(&h),
            Err(Challenge::Missing),
            "another scheme is no credentials"
        );
        for bad in ["Bearer", "Bearer a b", "Bearer ===", "Bearer a\"b"] {
            h.insert(header::AUTHORIZATION, HeaderValue::from_str(bad).unwrap());
            assert!(
                matches!(bearer(&h), Err(Challenge::InvalidRequest(_))),
                "{bad}"
            );
        }
        h.insert(header::AUTHORIZATION, HeaderValue::from_static("Bearer a"));
        h.append(header::AUTHORIZATION, HeaderValue::from_static("Bearer b"));
        assert!(matches!(bearer(&h), Err(Challenge::InvalidRequest(_))));
    }

    #[test]
    fn scopes_from_scope_or_scp() {
        assert_eq!(scopes(&serde_json::json!({"scope": "a  b"})), ["a", "b"]);
        assert_eq!(scopes(&serde_json::json!({"scp": ["a", "b"]})), ["a", "b"]);
        assert_eq!(scopes(&serde_json::json!({"scp": "a"})), ["a"]);
        assert_eq!(scopes(&serde_json::json!({})), Vec::<String>::new());
    }

    #[test]
    fn quoted_values_drop_quotes_and_backslashes() {
        assert_eq!(quoted("a\"b\\c\nd"), "abcd");
    }
}
