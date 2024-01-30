# Contribution guidelines

First off, thank you for considering contributing to the project `rs-measures`, including the crate `rs-measures` and the crate `units-relation`.

If your contribution is not straightforward, please first discuss the change you wish to make, by creating a new issue before creating the pull request.

## Reporting issues

Before reporting an issue on the [issue tracker](https://github.com/carlomilanesi/rs-measures/issues), please check that it has not already been reported by searching for some related keywords.

## Pull requests

Try to do one pull request per change.

### Updating the changelog

Update the document [CHANGELOG](https://github.com/carlomilanesi/rs-measures/blob/master/CHANGELOG.md) with the changes you have made under the **Unreleased** section.

Add the changes of your pull request to one of the following subsections, depending on the types of changes defined by [Keep a changelog](https://keepachangelog.com/en/1.0.0/):
- `Added` for new features.
- `Changed` for changes in existing functionality.
- `Deprecated` for soon-to-be removed features.
- `Removed` for now removed features.
- `Fixed` for any bug fixes.
- `Security` in case of vulnerabilities.

If the required subsection does not exist yet under **Unreleased**, create it.

## Developing

### Set up

This is no different than other Rust projects.

```shell
git clone https://github.com/carlomilanesi/rs-measures
cd rs-measures
cargo build
```

### Useful Commands

In the repository root, you can type these commands:

```sh
# Perform a quick performance check
cargo r --release --example bench

# Exercise all code, showing use cases.
cargo r --example full

# Check syntax
cargo c

# Lint examples
cargo clippy --examples

#  Run all tests
cargo t
```

The crate rs-measures consists essentially in macros.
The command `cargo fmt` cannot format the code inside such macros.
In particular, the source files containing only macros are:
* The files whose pathname starts with `define_`.
* The files contained in the folder `inner`, except for the file `mod.rs`.

So, to format that code, follow these process:
* For each of those files containing only macros, comment out the first three lines and the last two lines, and save the file.
* Run the command `cargo fmt`.
* For each of those files, uncomment the five lines commented in the first step.
* Select all the lines except the five just uncommented lines, and type twice the Tab key, to indent them, and save the file.

The command `format_measures.sh` performs this processing for all the macro files.

Before committing any change, except for those in documentation only, this script must be run successfully from the root folder of the repository:
```sh
./build.sh
```
