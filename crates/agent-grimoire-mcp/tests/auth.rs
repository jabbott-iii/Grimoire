//! Access-token checks (RFC 9068 §4, RFC 8725, RFC 6750, RFC 9728).

#![allow(clippy::unwrap_used, reason = "tests fail loudly by design")]

mod support;

use agent_grimoire_mcp::auth::{AuthConfig, Authenticator, Challenge};
use http::{HeaderMap, HeaderValue, StatusCode, header};
use serde_json::{Value, json};
use support::{
    ISSUER, RESOURCE, ec_jwk, es256, good_claims, now, rs256, rsa_jwk, sign_es256, sign_rs256,
    write_jwks,
};

fn config(jwks: std::path::PathBuf) -> AuthConfig {
    AuthConfig {
        resource: RESOURCE.to_owned(),
        issuer: ISSUER.to_owned(),
        jwks,
        audiences: Vec::new(),
        required_scopes: vec!["docs:read".to_owned()],
        resource_name: Some("Project docs".to_owned()),
        allow_jwt_typ: false,
        leeway_secs: 30,
    }
}

fn authenticator(dir: &tempfile::TempDir) -> Authenticator {
    let jwks = write_jwks(dir.path(), &[rsa_jwk("rsa1"), ec_jwk("ec1")]);
    Authenticator::new(config(jwks)).unwrap()
}

fn bearer(token: &str) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {token}")).unwrap(),
    );
    headers
}

fn with(claims: &Value, key: &str, value: Value) -> Value {
    let mut claims = claims.clone();
    claims[key] = value;
    claims
}

fn without(claims: &Value, key: &str) -> Value {
    let mut claims = claims.clone();
    claims.as_object_mut().unwrap().remove(key);
    claims
}

fn invalid(auth: &Authenticator, token: &str) -> &'static str {
    match auth.check(&bearer(token)) {
        Err(Challenge::InvalidToken(why)) => why,
        other => panic!("expected invalid_token, got {other:?}"),
    }
}

#[test]
fn valid_es256_and_rs256_tokens_are_accepted() {
    let dir = tempfile::tempdir().unwrap();
    let auth = authenticator(&dir);
    for token in [es256(&good_claims()), rs256(&good_claims())] {
        let caller = auth.check(&bearer(&token)).unwrap();
        assert_eq!(caller.subject.as_deref(), Some("user-1"));
        assert_eq!(caller.client_id.as_deref(), Some("client-1"));
        assert_eq!(caller.scopes, ["docs:read", "other"]);
    }
    // `scp` as an array, and an `aud` list that contains the resource.
    let claims = with(
        &without(&good_claims(), "scope"),
        "scp",
        json!(["docs:read"]),
    );
    let claims = with(&claims, "aud", json!(["https://other.example", RESOURCE]));
    assert!(auth.check(&bearer(&es256(&claims))).is_ok());
}

#[test]
fn claims_are_checked() {
    let dir = tempfile::tempdir().unwrap();
    let auth = authenticator(&dir);
    let claims = good_claims();
    assert!(
        invalid(
            &auth,
            &es256(&with(&claims, "aud", json!("https://other.example/mcp")))
        )
        .contains("not issued for this server")
    );
    assert!(
        invalid(
            &auth,
            &es256(&with(&claims, "iss", json!("https://evil.example")))
        )
        .contains("another authorization server")
    );
    assert!(invalid(&auth, &es256(&with(&claims, "exp", json!(now() - 120)))).contains("expired"));
    assert!(
        invalid(&auth, &es256(&with(&claims, "nbf", json!(now() + 3600))))
            .contains("not valid yet")
    );
    // A token without `aud` is refused, not waved through.
    assert!(invalid(&auth, &es256(&without(&claims, "aud"))).contains("required claim"));
    assert!(invalid(&auth, &es256(&without(&claims, "iss"))).contains("required claim"));
    assert!(invalid(&auth, &es256(&without(&claims, "exp"))).contains("required claim"));
    // Within the leeway, an expired token still passes.
    assert!(
        auth.check(&bearer(&es256(&with(&claims, "exp", json!(now() - 5)))))
            .is_ok()
    );
}

#[test]
fn headers_and_keys_are_checked() {
    let dir = tempfile::tempdir().unwrap();
    let auth = authenticator(&dir);
    let claims = good_claims();
    // RFC 9068: only access tokens, by `typ`.
    for typ in [json!("JWT"), json!("id+jwt"), Value::Null] {
        let token = sign_es256(
            &json!({ "alg": "ES256", "typ": typ, "kid": "ec1" }),
            &claims,
        );
        assert!(invalid(&auth, &token).contains("typ"), "{typ}");
    }
    let typed = sign_es256(
        &json!({ "alg": "ES256", "typ": "application/AT+JWT", "kid": "ec1" }),
        &claims,
    );
    assert!(
        auth.check(&bearer(&typed)).is_ok(),
        "typ is compared case-insensitively"
    );
    // RFC 8725: the key decides the algorithm; no confusion between them.
    let confused = sign_es256(
        &json!({ "alg": "RS256", "typ": "at+jwt", "kid": "ec1" }),
        &claims,
    );
    assert_eq!(invalid(&auth, &confused), "no matching signing key");
    let confused = sign_rs256(
        &json!({ "alg": "ES256", "typ": "at+jwt", "kid": "rsa1" }),
        &claims,
    );
    assert_eq!(invalid(&auth, &confused), "no matching signing key");
    for alg in ["HS256", "none", "PS256", "EdDSA"] {
        let token = sign_es256(
            &json!({ "alg": alg, "typ": "at+jwt", "kid": "ec1" }),
            &claims,
        );
        assert!(
            matches!(auth.check(&bearer(&token)), Err(Challenge::InvalidToken(_))),
            "{alg}"
        );
    }
    let unknown = sign_es256(
        &json!({ "alg": "ES256", "typ": "at+jwt", "kid": "nope" }),
        &claims,
    );
    assert_eq!(invalid(&auth, &unknown), "no matching signing key");
    // Without `kid`, the one key for the algorithm is used.
    let no_kid = sign_es256(&json!({ "alg": "ES256", "typ": "at+jwt" }), &claims);
    assert!(auth.check(&bearer(&no_kid)).is_ok());
    // A tampered payload fails the signature.
    let token = es256(&claims);
    let mut parts: Vec<String> = token.split('.').map(str::to_owned).collect();
    parts[1] = sign_es256(&json!({}), &with(&claims, "sub", json!("admin")))
        .split('.')
        .nth(1)
        .unwrap()
        .to_owned();
    assert_eq!(invalid(&auth, &parts.join(".")), "invalid signature");
    assert_eq!(invalid(&auth, "not.a.jwt"), "malformed token");
}

#[test]
fn jwt_typ_can_be_allowed_explicitly() {
    let dir = tempfile::tempdir().unwrap();
    let jwks = write_jwks(dir.path(), &[ec_jwk("ec1")]);
    let auth = Authenticator::new(AuthConfig {
        allow_jwt_typ: true,
        ..config(jwks)
    })
    .unwrap();
    let token = sign_es256(
        &json!({ "alg": "ES256", "typ": "JWT", "kid": "ec1" }),
        &good_claims(),
    );
    assert!(auth.check(&bearer(&token)).is_ok());
}

#[test]
fn scopes_and_requests_are_checked() {
    let dir = tempfile::tempdir().unwrap();
    let auth = authenticator(&dir);
    let token = es256(&with(&good_claims(), "scope", json!("other")));
    assert_eq!(
        auth.check(&bearer(&token)),
        Err(Challenge::InsufficientScope)
    );
    assert_eq!(auth.check(&HeaderMap::new()), Err(Challenge::Missing));
    let mut basic = HeaderMap::new();
    basic.insert(
        header::AUTHORIZATION,
        HeaderValue::from_static("Basic dXNlcjpwYXNz"),
    );
    assert_eq!(
        auth.check(&basic),
        Err(Challenge::Missing),
        "another scheme gets a plain challenge"
    );
    let mut repeated = HeaderMap::new();
    repeated.insert(header::AUTHORIZATION, HeaderValue::from_static("Bearer a"));
    repeated.append(header::AUTHORIZATION, HeaderValue::from_static("Bearer b"));
    assert!(matches!(
        auth.check(&repeated),
        Err(Challenge::InvalidRequest(_))
    ));
}

#[test]
fn challenges_follow_rfc_6750_and_name_the_metadata() {
    let dir = tempfile::tempdir().unwrap();
    let auth = authenticator(&dir);
    let metadata =
        "resource_metadata=\"https://docs.example.com/.well-known/oauth-protected-resource/mcp\"";
    let (status, value) = auth.challenge(&Challenge::Missing);
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert_eq!(
        value.to_str().unwrap(),
        format!("Bearer {metadata}, scope=\"docs:read\"")
    );
    let (status, value) = auth.challenge(&Challenge::InvalidToken("the token has expired"));
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(
        value
            .to_str()
            .unwrap()
            .ends_with("error=\"invalid_token\", error_description=\"the token has expired\"")
    );
    let (status, value) = auth.challenge(&Challenge::InsufficientScope);
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert!(
        value
            .to_str()
            .unwrap()
            .contains("scope=\"docs:read\", error=\"insufficient_scope\"")
    );
    let (status, _) = auth.challenge(&Challenge::InvalidRequest("x"));
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[test]
fn metadata_is_rfc_9728() {
    let dir = tempfile::tempdir().unwrap();
    let auth = authenticator(&dir);
    assert_eq!(
        auth.metadata_path(),
        "/.well-known/oauth-protected-resource/mcp"
    );
    assert_eq!(
        auth.metadata(),
        json!({
            "resource": RESOURCE,
            "authorization_servers": [ISSUER],
            "scopes_supported": ["docs:read"],
            "bearer_methods_supported": ["header"],
            "resource_name": "Project docs",
        })
    );
    let jwks = write_jwks(dir.path(), &[ec_jwk("ec1")]);
    let root = Authenticator::new(AuthConfig {
        resource: "https://docs.example.com".to_owned(),
        required_scopes: Vec::new(),
        resource_name: None,
        ..config(jwks)
    })
    .unwrap();
    assert_eq!(
        root.metadata_path(),
        "/.well-known/oauth-protected-resource"
    );
    assert!(
        root.metadata().get("scopes_supported").is_none(),
        "empty fields are omitted"
    );
}

#[test]
fn unusable_keys_and_bad_settings_are_refused_at_startup() {
    let dir = tempfile::tempdir().unwrap();
    let k = support::keys();
    // A 1024-bit RSA key, a key for encryption, an RS384-only key and a P-384 key are all skipped.
    let short_n = {
        use base64::Engine as _;
        let n = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(k["rsa"]["n"].as_str().unwrap())
            .unwrap();
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(&n[..128])
    };
    let unusable = [
        json!({ "kty": "RSA", "kid": "short", "n": short_n, "e": "AQAB" }),
        json!({ "kty": "RSA", "kid": "enc", "use": "enc", "n": k["rsa"]["n"], "e": "AQAB" }),
        json!({ "kty": "RSA", "kid": "384", "alg": "RS384", "n": k["rsa"]["n"], "e": "AQAB" }),
        json!({ "kty": "EC", "kid": "p384", "crv": "P-384", "x": k["ec"]["x"], "y": k["ec"]["y"] }),
    ];
    let jwks = write_jwks(dir.path(), &unusable);
    let error = Authenticator::new(config(jwks)).unwrap_err();
    assert!(
        error.to_string().contains("no usable signing key"),
        "{error}"
    );

    let jwks = write_jwks(dir.path(), &[ec_jwk("ec1")]);
    for bad in [
        AuthConfig {
            leeway_secs: 61,
            ..config(jwks.clone())
        },
        AuthConfig {
            resource: "http://docs.example.com/mcp".to_owned(),
            ..config(jwks.clone())
        },
        AuthConfig {
            resource: "https://docs.example.com/mcp?x=1".to_owned(),
            ..config(jwks.clone())
        },
        AuthConfig {
            issuer: " ".to_owned(),
            ..config(jwks.clone())
        },
        AuthConfig {
            required_scopes: vec!["a b".to_owned()],
            ..config(jwks.clone())
        },
        AuthConfig {
            jwks: dir.path().join("missing.json"),
            ..config(jwks.clone())
        },
    ] {
        assert!(Authenticator::new(bad).is_err());
    }
    // Plain http is allowed for a loopback resource, for local development.
    assert!(
        Authenticator::new(AuthConfig {
            resource: "http://127.0.0.1:8931/mcp".to_owned(),
            ..config(jwks)
        })
        .is_ok()
    );
}

#[test]
fn a_changed_key_file_is_reloaded_and_a_broken_one_ignored() {
    let dir = tempfile::tempdir().unwrap();
    let jwks = write_jwks(dir.path(), &[ec_jwk("ec1")]);
    let auth = Authenticator::new(config(jwks)).unwrap();
    assert!(
        auth.check(&bearer(&rs256(&good_claims()))).is_err(),
        "no RSA key yet"
    );
    std::thread::sleep(std::time::Duration::from_millis(5100));
    write_jwks(dir.path(), &[rsa_jwk("rsa1")]);
    assert!(
        auth.check(&bearer(&rs256(&good_claims()))).is_ok(),
        "the new key is used"
    );
    std::thread::sleep(std::time::Duration::from_millis(5100));
    std::fs::write(dir.path().join("jwks.json"), "not json").unwrap();
    assert!(
        auth.check(&bearer(&rs256(&good_claims()))).is_ok(),
        "the previous keys stay"
    );
}

#[test]
fn iss_must_be_the_issuer_string_not_a_list_containing_it() {
    let dir = tempfile::tempdir().unwrap();
    let auth = authenticator(&dir);
    let listed = with(
        &good_claims(),
        "iss",
        json!([ISSUER, "https://evil.example"]),
    );
    assert!(invalid(&auth, &es256(&listed)).contains("another authorization server"));
    let only = with(&good_claims(), "iss", json!([ISSUER]));
    assert!(invalid(&auth, &es256(&only)).contains("another authorization server"));
}

#[test]
fn extra_audiences_are_accepted_besides_the_resource() {
    let dir = tempfile::tempdir().unwrap();
    let jwks = write_jwks(dir.path(), &[ec_jwk("ec1")]);
    let auth = Authenticator::new(AuthConfig {
        audiences: vec!["api://docs".to_owned()],
        ..config(jwks)
    })
    .unwrap();
    assert!(
        auth.check(&bearer(&es256(&good_claims()))).is_ok(),
        "the resource itself"
    );
    let custom = with(&good_claims(), "aud", json!("api://docs"));
    assert!(
        auth.check(&bearer(&es256(&custom))).is_ok(),
        "a configured audience"
    );
    let other = with(&good_claims(), "aud", json!("api://other"));
    assert!(invalid(&auth, &es256(&other)).contains("not issued for this server"));
}

#[test]
fn emptying_or_deleting_the_key_file_revokes_the_keys() {
    let dir = tempfile::tempdir().unwrap();
    let jwks = write_jwks(dir.path(), &[ec_jwk("ec1")]);
    let auth = Authenticator::new(config(jwks.clone())).unwrap();
    assert!(auth.check(&bearer(&es256(&good_claims()))).is_ok());
    std::thread::sleep(std::time::Duration::from_millis(5100));
    write_jwks(dir.path(), &[]);
    assert_eq!(
        invalid(&auth, &es256(&good_claims())),
        "no matching signing key",
        "an empty set revokes"
    );
    std::thread::sleep(std::time::Duration::from_millis(5100));
    write_jwks(dir.path(), &[ec_jwk("ec1")]);
    assert!(
        auth.check(&bearer(&es256(&good_claims()))).is_ok(),
        "keys come back with the file"
    );
    std::thread::sleep(std::time::Duration::from_millis(5100));
    std::fs::remove_file(&jwks).unwrap();
    assert_eq!(
        invalid(&auth, &es256(&good_claims())),
        "no matching signing key",
        "a deleted file revokes"
    );
}
