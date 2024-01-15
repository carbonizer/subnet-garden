// Copyright 2023-2024 The Milton Hirsch Institute, B.V.
// SPDX-License-Identifier: Apache-2.0

#![cfg(feature = "cli")]

mod test_subcommands;
mod tests;

use subnet_garden::subg::args::Subg;
use tests::new_test;
use tests::HELP_EXIT_CODE;

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Subg::command().debug_assert();
}
#[test]
fn test_bin() {
    let mut test = new_test();
    test.subg
        .assert()
        .failure()
        .code(HELP_EXIT_CODE)
        .stderr(predicates::str::contains(
            "\'subg\' requires a subcommand but one was not provided",
        ));
}
