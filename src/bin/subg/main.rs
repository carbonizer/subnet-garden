// Copyright 2023-2024 The Milton Hirsch Institute, B.V.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser as _;
use subnet_garden::subg::args::{Subg, SubgCommands};
use subnet_garden::subg::subcommands::init;
use subnet_garden::subg::subcommands::subnet;

fn main() {
    let subg = Subg::parse();

    match subg.command {
        SubgCommands::Init(args) => {
            init::init(&subg.args, &args);
        }
        SubgCommands::Allocate(args) => {
            subnet::allocate(&subg.args, &args);
        }
        SubgCommands::Free(args) => {
            subnet::free(&subg.args, &args);
        }
        SubgCommands::Cidrs(args) => {
            subnet::cidrs(&subg.args, &args);
        }
        SubgCommands::Names(args) => {
            subnet::names(&subg.args, &args);
        }
        SubgCommands::Claim(args) => {
            subnet::claim(&subg.args, &args);
        }
        SubgCommands::Rename(args) => {
            subnet::rename(&subg.args, &args);
        }
        SubgCommands::MaxAvailable(_) => {
            subnet::max_bits(&subg.args);
        }
    }
}
