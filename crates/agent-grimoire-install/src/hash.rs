//! Content hashes recorded in `grimoire.lock` (`architecture.md` §6.5, §7.3).
//!
//! Text is hashed with `\r\n` line endings folded to `\n`, so a checkout that
//! converts line endings (for example `core.autocrlf` on Windows) is not reported
//! as drift. Content that is not UTF-8, or that contains a NUL byte, is binary and
//! hashed exactly; Git uses the same NUL test to tell binary files from text.

use agent_grimoire_model::Sha256Digest;

/// The hash of file or block content.
#[must_use]
pub fn content(bytes: &[u8]) -> Sha256Digest {
    match std::str::from_utf8(bytes) {
        Ok(text) if !text.contains('\0') && text.contains("\r\n") => {
            Sha256Digest::of(text.replace("\r\n", "\n").as_bytes())
        }
        _ => Sha256Digest::of(bytes),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn text_ignores_crlf_but_binary_does_not() {
        assert_eq!(content(b"a\r\nb\r\n"), content(b"a\nb\n"));
        assert_ne!(content(b"a\r\n\0"), content(b"a\n\0"));
        assert_ne!(content(&[0xff, b'\r', b'\n']), content(&[0xff, b'\n']));
        assert_ne!(content(b"a\n"), content(b"a"));
    }
}
