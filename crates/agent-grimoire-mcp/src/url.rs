//! A strict parser for the canonical URI of an MCP endpoint.

/// A parsed `https://host[:port]/path` (or `http://` for this machine only).
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Endpoint {
    /// `https` or `http`.
    pub(crate) scheme: String,
    /// The host, lowercased; IPv6 in brackets.
    pub(crate) host: String,
    /// The explicit port, if any.
    pub(crate) port: Option<u16>,
    /// The path, starting with `/` (just `/` when absent).
    pub(crate) path: String,
}

/// Whether a host names this machine.
pub(crate) fn is_loopback_host(host: &str) -> bool {
    let host = host.to_ascii_lowercase();
    host == "localhost"
        || host == "[::1]"
        || host.strip_prefix("127.").is_some_and(|rest| {
            rest.split('.').count() == 3 && rest.split('.').all(|p| p.parse::<u8>().is_ok())
        })
}

impl Endpoint {
    /// Parses a canonical endpoint URI: no user information, query or fragment
    /// (RFC 9728 §1.2, OAuth 2.1 §1.5), and `http` only for this machine.
    pub(crate) fn parse(value: &str) -> Result<Self, String> {
        let (scheme, rest) = value
            .split_once("://")
            .ok_or_else(|| format!("`{value}` is not an absolute https:// URI"))?;
        let scheme = scheme.to_ascii_lowercase();
        if scheme != "https" && scheme != "http" {
            return Err(format!("`{value}` must use https://"));
        }
        if rest.contains(['?', '#', '@', ' ', '\\']) || !rest.is_ascii() {
            return Err(format!(
                "`{value}` must not have user information, a query or a fragment"
            ));
        }
        let (authority, path) = rest.find('/').map_or((rest, "/"), |i| rest.split_at(i));
        let (host, port) = match authority.rfind(':') {
            Some(i) if !authority[i..].contains(']') => {
                let port = authority[i + 1..]
                    .parse::<u16>()
                    .ok()
                    .filter(|p| *p > 0)
                    .ok_or_else(|| format!("`{value}` has an invalid port"))?;
                (&authority[..i], Some(port))
            }
            _ => (authority, None),
        };
        if host.is_empty()
            || !host
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"-.[]:".contains(&b))
        {
            return Err(format!("`{value}` has an invalid host"));
        }
        if scheme == "http" && !is_loopback_host(host) {
            return Err(format!(
                "`{value}` must use https://; plain http:// is only for localhost, 127.0.0.0/8 and [::1]"
            ));
        }
        Ok(Self {
            scheme,
            host: host.to_ascii_lowercase(),
            port,
            path: path.to_owned(),
        })
    }

    /// `scheme://host[:port]`.
    pub(crate) fn origin(&self) -> String {
        match self.port {
            Some(port) => format!("{}://{}:{port}", self.scheme, self.host),
            None => format!("{}://{}", self.scheme, self.host),
        }
    }

    /// The `Host` header values a client sends for this endpoint.
    pub(crate) fn host_headers(&self) -> Vec<String> {
        let default = if self.scheme == "https" { 443 } else { 80 };
        match self.port {
            Some(port) if port != default => vec![format!("{}:{port}", self.host)],
            _ => vec![self.host.clone(), format!("{}:{default}", self.host)],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoints() {
        let e = Endpoint::parse("https://Docs.Example.com/mcp").unwrap();
        assert_eq!(
            (e.host.as_str(), e.port, e.path.as_str()),
            ("docs.example.com", None, "/mcp")
        );
        assert_eq!(e.origin(), "https://docs.example.com");
        assert_eq!(
            e.host_headers(),
            ["docs.example.com", "docs.example.com:443"]
        );
        let local = Endpoint::parse("http://127.0.0.1:8931/mcp").unwrap();
        assert_eq!(local.host_headers(), ["127.0.0.1:8931"]);
        assert_eq!(Endpoint::parse("https://[::1]:8443").unwrap().path, "/");
        for bad in [
            "docs.example.com/mcp",
            "ftp://x/mcp",
            "http://docs.example.com/mcp",
            "https://u@docs.example.com/mcp",
            "https://docs.example.com/mcp?x=1",
            "https://docs.example.com/mcp#f",
            "https://docs.example.com:0/mcp",
            "https://:443/mcp",
        ] {
            assert!(Endpoint::parse(bad).is_err(), "{bad}");
        }
        assert!(
            is_loopback_host("LOCALHOST")
                && is_loopback_host("127.0.0.1")
                && !is_loopback_host("127.0.0.1.example")
        );
    }
}
