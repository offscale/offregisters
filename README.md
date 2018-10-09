# Summary
This is a collection (Workspace in Rust-specific terminology) of small crates designed to provide bits of functionality for installing, configuring, and managing applications.

## Library Crates
These crates are meant to be used by binary crates that manage a specific task. All library crates follow the naming convention of: `lib_<functionality>`. See the README.md file in each library crate for details about its functionality.

## Binary Crates
These crates are leverage the library crates to do whatever is needed to complete its task.
