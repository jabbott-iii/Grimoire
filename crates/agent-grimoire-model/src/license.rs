//! SPDX license expressions.

use crate::constrained::constrained_string;

fn check_license(value: &str) -> Result<(), String> {
    // STRICT parsing: rejects unknown, deprecated and lowercase identifiers and `/` as OR.
    spdx::Expression::parse(value).map_err(|e| e.to_string())?;
    // STRICT still accepts lowercase operators and irregular whitespace; require the canonical form.
    match spdx::Expression::canonicalize(value) {
        Ok(None) => Ok(()),
        Ok(Some(canonical)) => Err(format!("must be written in canonical form: {canonical:?}")),
        Err(e) => Err(e.to_string()),
    }
}

constrained_string! {
    /// A valid SPDX license expression, e.g. `MIT OR Apache-2.0`.
    pub struct LicenseExpr {
        what: "license expression",
        check: check_license,
        schema: {
            "type": "string",
            "minLength": 1,
            "description": "SPDX license expression, e.g. \"MIT OR Apache-2.0\"."
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expressions() {
        assert!(LicenseExpr::new("MIT OR Apache-2.0").is_ok());
        assert!(LicenseExpr::new("Apache-2.0 WITH LLVM-exception").is_ok());
        assert!(LicenseExpr::new("MIT/Apache-2.0").is_err());
        assert!(LicenseExpr::new("Not-A-License").is_err());
        assert!(LicenseExpr::new("").is_err());
        assert!(
            LicenseExpr::new("GPL-2.0").is_err(),
            "deprecated identifier"
        );
        assert!(
            LicenseExpr::new("MIT or Apache-2.0").is_err(),
            "lowercase operator"
        );
        assert!(LicenseExpr::new(" MIT").is_err(), "surrounding whitespace");
        assert!(
            LicenseExpr::new("MIT  OR  Apache-2.0").is_err(),
            "irregular whitespace"
        );
    }
}
