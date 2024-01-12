# About
This project will hold a small application that will take
the provided commit message and the last semantic version and will produce
a new semantic version according to what is defined in the [conventional commits standard](https://www.conventionalcommits.org/en/v1.0.0/#summary)

This tool is similar to [git-semver](https://github.com/PSanetra/git-semver) with the difference
that it does not rely on tags on the commit history.

The idea is to use this small application to increase the semver of your project with a commit hook 
like the ones intalled with husky.

## Installation
1. Run cargo build -r
2. Copy the executable from the `target/release` into your path.

## Usage

```bash
next_version=$(semver-next --commit-message "feat: some commit message" --last-version "1.2.3"`)
echo $next_version

```
