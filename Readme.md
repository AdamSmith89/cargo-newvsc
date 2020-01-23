[![Build Status](https://travis-ci.org/AdamSmith89/cargo-newvsc.svg?branch=master)](https://travis-ci.org/AdamSmith89/cargo-newvsc)
# Overview
Drop-in replacement for `cargo new` when using VS Code.

First uses `cargo new` to initialise a new package, with support for all parameters. Then deploys a basic `tasks.json` file with build & test tasks.
# Usage
`cargo install cargo-newvsc`

`cargo newvsc`
# To Do
- See if it can become `cargo newvsc` (without the dash) - might have to be a lib?
- Further tasks or maybe a launch.json