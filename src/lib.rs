mod commit_message_validator;
mod semver_validator;

use commit_message_validator::validate_commit_message;
use commit_message_validator::VersionChangeType;
use semver_validator::validate_semver;

pub fn generate_version(commit_message: String, last_version: String) -> Result<String, String> {
    eprintln!("Commit message: {commit_message}");
    eprintln!("Last semantic version: {last_version}");

    validate_semver(&last_version)?;
    let next_semver = validate_commit_message(&commit_message)?;

    match next_semver {
        VersionChangeType::Major => {
            let mut split = last_version.split('.');
            let major = split.next().unwrap().parse::<u32>().unwrap() + 1;
            let minor = 0;
            let patch = 0;
            Ok(format!("{}.{}.{}", major, minor, patch))
        }
        VersionChangeType::Minor => {
            let mut split = last_version.split('.');
            let major = split.next().unwrap().parse::<u32>().unwrap();
            let minor = split.next().unwrap().parse::<u32>().unwrap() + 1;
            let patch = 0;
            Ok(format!("{}.{}.{}", major, minor, patch))
        }
        VersionChangeType::Patch => {
            let mut split = last_version.split('.');
            let major = split.next().unwrap().parse::<u32>().unwrap();
            let minor = split.next().unwrap().parse::<u32>().unwrap();
            let patch = split.next().unwrap().parse::<u32>().unwrap() + 1;
            Ok(format!("{}.{}.{}", major, minor, patch))
        }
        VersionChangeType::None => Ok(last_version),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("feat!: major change")]
    #[case("feat(scoped)!: major change")]
    #[case("feat: major change\nBREAKING CHANGE")]
    fn should_generate_with_major_change(#[case] commit_message: &str) {
        let last_version = "1.0.0".to_string();
        let expected = "2.0.0".to_string();
        let actual = generate_version(commit_message.to_string(), last_version).unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("feat: minor change")]
    #[case("feat(scoped): minor change")]
    fn should_generate_with_minor_change(#[case] commit_message: &str) {
        let last_version = "1.0.0".to_string();
        let expected = "1.1.0".to_string();
        let actual = generate_version(commit_message.to_string(), last_version).unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("fix: patch change")]
    #[case("fix(scoped): patch change")]
    fn should_generate_with_patch_change(#[case] commit_message: &str) {
        let last_version = "1.0.0".to_string();
        let expected = "1.0.1".to_string();
        let actual = generate_version(commit_message.to_string(), last_version).unwrap();
        assert_eq!(expected, actual);
    }

    #[rstest]
    #[case("chore: no change")]
    #[case("chore(scoped): no change")]
    fn should_generate_with_no_change(#[case] commit_message: &str) {
        let last_version = "1.0.0".to_string();
        let expected = "1.0.0".to_string();
        let actual = generate_version(commit_message.to_string(), last_version).unwrap();
        assert_eq!(expected, actual);
    }
}
