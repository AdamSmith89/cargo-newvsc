[![Build Status](https://travis-ci.org/AdamSmith89/cargo-newvsc.svg?branch=master)](https://travis-ci.org/AdamSmith89/cargo-newvsc)
# Overview
Drop-in replacement for `cargo new` when using VS Code.

First uses `cargo new` to initialise a new package, with support for all parameters. Then deploys a basic `tasks.json` file with build, run & test tasks.

# Dependencies
[Rust (rls)](https://github.com/rust-lang/rls-vscode) VSCode extension.

# Install
`cargo install cargo-newvsc`

# Usage
`cargo newvsc <any cargo new params>`