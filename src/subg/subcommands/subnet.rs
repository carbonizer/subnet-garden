// Copyright 2023-2024 The Milton Hirsch Institute, B.V.
// SPDX-License-Identifier: Apache-2.0

use crate::subg::args::{
    AllocateArgs, CidrsArgs, ClaimArgs, FreeArgs, NamesArgs, RenameArgs, SubgArgs,
};
use crate::subg::pool as subg_pool;
use cidr::IpCidr;
use std::process::exit;

pub fn allocate(subg: &SubgArgs, args: &AllocateArgs) {
    let mut pool = subg_pool::load_pool(&subg.pool_path);
    subg_pool::result(
        pool.allocate(args.bits, args.name.as_deref()),
        exitcode::SOFTWARE,
        "Could not allocate subnet",
    );
    subg_pool::store_pool(&subg.pool_path, &pool);
}

pub fn free(subg: &SubgArgs, args: &FreeArgs) {
    let mut pool = subg_pool::load_pool(&subg.pool_path);
    let cidr = match pool.find_by_name(args.identifier.as_str()) {
        Some(cidr) => cidr,
        None => subg_pool::result(
            args.identifier.parse::<IpCidr>(),
            exitcode::USAGE,
            "Could not parse arg IDENTIFIER",
        ),
    };
    if !pool.free(&cidr) {
        eprintln!("Could not free subnet {}", cidr);
        exit(exitcode::SOFTWARE);
    }
    subg_pool::store_pool(&subg.pool_path, &pool);
}

pub fn cidrs(subg: &SubgArgs, args: &CidrsArgs) {
    let pool = subg_pool::load_pool(&subg.pool_path);

    if args.long {
        println!("total {}", pool.allocated_count());
    }

    let max_cidr_width = match args.long {
        true => pool
            .records()
            .map(|r| r.cidr.to_string().len())
            .max()
            .unwrap_or(0),
        false => 0,
    };
    for entry in pool.records() {
        let cidr = entry.cidr.to_string();
        if args.long {
            let cidr = format! {"{cidr:<max_cidr_width$}"};
            let name = entry.name.clone().unwrap_or("-".to_string());
            println!("{}  {}", cidr, name);
        } else {
            println!("{}", cidr);
        }
    }
}

pub fn names(subg: &SubgArgs, args: &NamesArgs) {
    let pool = subg_pool::load_pool(&subg.pool_path);

    if args.long {
        println!("total {} of {}", pool.named_count(), pool.allocated_count());
    }

    let max_name_width = match args.long {
        true => pool.names().map(|n| n.len()).max().unwrap_or(0),
        false => 0,
    };

    let mut names: Vec<String> = pool.names().collect();
    names.sort();
    for name in names {
        if args.long {
            let cidr = pool.find_by_name(&name).unwrap();
            let cidr_string = cidr.to_string();
            let name = format!("{name:<max_name_width$}");
            println!("{}  {}", name, cidr_string);
        } else {
            println!("{}", name);
        }
    }
}

pub fn claim(subg: &SubgArgs, args: &ClaimArgs) {
    let mut pool = subg_pool::load_pool(&subg.pool_path);
    subg_pool::result(
        pool.claim(&args.cidr, args.name.as_deref()),
        exitcode::SOFTWARE,
        "Could not claim subnet",
    );
    subg_pool::store_pool(&subg.pool_path, &pool);
}

pub fn rename(subg: &SubgArgs, args: &RenameArgs) {
    let mut pool = subg_pool::load_pool(&subg.pool_path);
    let cidr = match pool.find_by_name(args.identifier.as_str()) {
        Some(cidr) => cidr,
        None => subg_pool::result(
            args.identifier.parse::<IpCidr>(),
            exitcode::USAGE,
            "Could not parse arg IDENTIFIER",
        ),
    };
    subg_pool::result(
        pool.rename(&cidr, args.name.as_deref()),
        exitcode::SOFTWARE,
        "Could not rename subnet",
    );
    subg_pool::store_pool(&subg.pool_path, &pool);
}

pub fn max_bits(subg: &SubgArgs) {
    let pool = subg_pool::load_pool(&subg.pool_path);
    let largest = pool.max_available_bits();
    println!("{}", largest);
}
