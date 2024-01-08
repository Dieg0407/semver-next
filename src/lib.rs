pub fn generate_version(commit_message: String, last_version: String) -> String {
    println!("Commit message: {commit_message}");
    println!("Last semantic version: {last_version}");

    last_version
}
