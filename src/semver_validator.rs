use regex::Regex;

pub fn validate_semver(version: &str) -> Result<(), String> {
    let semver_patter = Regex::new(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)$").unwrap();

    if !semver_patter.is_match(version) {
        return Err(format!(
            "Version {} is not a valid semver version. Please use the format X.Y.Z",
            version
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_fail() {
        let invalid_versions = ["1.0", "1.0.0a", "v1.0.0", "01.0.0", "1.0.0-alpha+001"];

        for version in invalid_versions {
            let result = validate_semver(version);
            assert!(result.is_err());
        }
    }

    #[test]
    fn should_pass() {
        let valid_versions = ["1.0.0", "1.0.0", "1.0.1", "3.4.12"];
        for version in valid_versions {
            let result = validate_semver(version);
            assert!(result.is_ok());
        }
    }
}
