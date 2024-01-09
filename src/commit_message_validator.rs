const VALID_STARTS: [&'static str; 10] = [
    "feat", "fix", "docs", "style", "build", "refactor", "perf", "test", "chore", "revert",
];

#[derive(Debug, PartialEq)]
pub enum ValidateCommitResult {
    Major,
    Minor,
    Patch,
    None,
}

pub fn validate_commit_message(commit_message: &str) -> Result<ValidateCommitResult, String> {
    if !commit_message_is_formated(&commit_message) {
        return Err("Commit message does not start with a valid type".to_string());
    }

    if commit_message.contains("BREAKING CHANGE") {
        return Ok(ValidateCommitResult::Major);
    }
    if commit_message.starts_with("feat!:") {
        return Ok(ValidateCommitResult::Major);
    }
    if commit_message.starts_with("feat(") && commit_message.contains(")!:") {
        return Ok(ValidateCommitResult::Major);
    }
    if commit_message.starts_with("feat:") {
        return Ok(ValidateCommitResult::Minor);
    }
    if commit_message.starts_with("feat(") && commit_message.contains("):") {
        return Ok(ValidateCommitResult::Minor);
    }
    if commit_message.starts_with("fix:") {
        return Ok(ValidateCommitResult::Patch);
    }
    if commit_message.starts_with("fix(") && commit_message.contains("):") {
        return Ok(ValidateCommitResult::Patch);
    }

    Ok(ValidateCommitResult::None)
}

fn commit_message_is_formated(commit_message: &str) -> bool {
    let mut matches_one = false;
    for i in VALID_STARTS.iter() {
        let valid_start_01 = format!("{}:", i);
        let valid_start_02 = format!("{}!:", i);
        let valid_start_03 = format!("{}(", i);
        if commit_message.starts_with(&valid_start_01) {
            matches_one = true;
            break;
        }
        if commit_message.starts_with(&valid_start_02) {
            matches_one = true;
            break;
        }
        if commit_message.starts_with(&valid_start_03) && commit_message.contains("):") {
            matches_one = true;
            break;
        }
        if commit_message.starts_with(&valid_start_03) && commit_message.contains(")!:") {
            matches_one = true;
            break;
        }
    }

    return matches_one;
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("feat: add new feature")]
    #[case("fix: fix a bug")]
    #[case("docs: document something")]
    #[case("style: fix whitespace")]
    #[case("build: build something")]
    #[case("refactor: change a lot of code")]
    #[case("perf: improve performance")]
    #[case("test: add missing tests")]
    #[case("chore: something else")]
    #[case("revert: revert something")]
    fn assert_commit_message_starts_with_valid_type(#[case] commit_message: &str) {
        let result = validate_commit_message(commit_message);
        assert!(result.is_ok());
    }

    #[rstest]
    #[case("this is not a valid conventional commit message")]
    #[case("this message doesn't start with feat: ")]
    #[case("feat[scoped]: this is an incorrect scoped version")]
    fn assert_commit_message_is_not_valid_type(#[case] commit_message: &str) {
        let result = validate_commit_message(commit_message);
        assert!(result.is_err());
    }

    #[rstest]
    #[case("feat!: add new feature")]
    #[case("feat(scoped)!: add new feature")]
    #[case(
        "feat: some other change

BREAKING CHANGE: some information about this breaking change"
    )]
    fn assert_commit_message_is_major_change(#[case] commit_message: &str) {
        let result = validate_commit_message(commit_message);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ValidateCommitResult::Major);
    }

    #[rstest]
    #[case("feat: add new feature")]
    #[case("feat(scoped): new scoped feature")]
    fn assert_commit_message_is_minor_change(#[case] commit_message: &str) {
        let result = validate_commit_message(commit_message);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ValidateCommitResult::Minor);
    }

    #[rstest]
    #[case("fix: fix a bug")]
    #[case("fix(scoped): fix a scoped bug")]
    fn assert_commit_message_is_patch_change(#[case] commit_message: &str) {
        let result = validate_commit_message(commit_message);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), ValidateCommitResult::Patch);
    }
}
