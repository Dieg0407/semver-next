mod commit_message_validator;
mod semver_validator;

use commit_message_validator::validate_commit_message;
use semver_validator::validate_semver;

pub fn generate_version(commit_message: String, last_version: String) -> Result<String, String> {
    eprintln!("Commit message: {commit_message}");
    eprintln!("Last semantic version: {last_version}");

    validate_semver(&last_version)?;
    validate_commit_message(&commit_message)?;

    Ok(last_version)
}
