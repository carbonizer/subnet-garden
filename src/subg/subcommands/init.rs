// Copyright 2023-2024 The Milton Hirsch Institute, B.V.
// SPDX-License-Identifier: Apache-2.0

use crate::pool;
use crate::subg;
use crate::subg::args::init::InitArgs;
use crate::subg::args::SubgArgs;
use std::path::Path;
use std::process::exit;

pub fn init(subg: &SubgArgs, args: &InitArgs) {
    let path = Path::new(&subg.pool_path);
    if path.exists() {
        if !args.force {
            eprintln!("Pool file already exists at {}", path.display());
            exit(exitcode::CANTCREAT);
        }
        if !path.is_file() {
            eprintln!("Path is not a file at {}", path.display());
            exit(exitcode::CANTCREAT);
        }
    }
    subg::pool::store_pool(&subg.pool_path, &pool::SubnetPool::new(args.cidr));
}
