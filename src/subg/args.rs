// Copyright 2023-2024 The Milton Hirsch Institute, B.V.
// SPDX-License-Identifier: Apache-2.0

use crate::Bits;
use cidr::IpCidr;

pub const DEFAULT_STORAGE_PATH: &str = "subnet-garden-pool.yaml";

pub const SUBG_COMMAND: &str = "subg";

pub mod init {
    use cidr::IpCidr;

    #[derive(Debug, clap::Args)]
    /// Initialize the subnet garden pool file
    pub struct InitArgs {
        #[arg(short, long, default_value_t)]
        /// Force initialization even if the pool file already exists
        pub force: bool,

        #[arg()]
        /// Pool subnet CIDR
        pub cidr: IpCidr,
    }
}

#[derive(Debug, clap::Args)]
/// Allocate subnet
pub struct AllocateArgs {
    #[arg()]
    /// Number of subnet bits
    pub bits: Bits,

    #[arg()]
    /// Name of the subnet to allocate
    pub name: Option<String>,
}

#[derive(Debug, clap::Args)]
/// Free subnet
pub struct FreeArgs {
    #[arg()]
    /// Name or CIDR of a subnet
    pub identifier: String,
}

#[derive(Debug, clap::Args)]
/// List allocate CIDRs
pub struct CidrsArgs {
    #[arg(short)]
    /// List CIDRs in long format
    pub long: bool,
}

#[derive(Debug, clap::Args)]
/// List named subnets
pub struct NamesArgs {
    #[arg(short)]
    /// List named CIDRs in long format
    pub long: bool,
}

#[derive(Debug, clap::Args)]
/// Claim subnet
pub struct ClaimArgs {
    #[arg()]
    /// CIDR subnet to claim
    pub cidr: IpCidr,

    #[arg()]
    /// Name of the subnet to claim
    pub name: Option<String>,
}

#[derive(Debug, clap::Args)]
/// Rename subnet
pub struct RenameArgs {
    #[arg()]
    /// Name or CIDR of the subnet to rename
    pub identifier: String,

    #[arg()]
    /// New name of the subnet or omit to remove the name
    pub name: Option<String>,
}

#[derive(Debug, clap::Args)]
/// Largest available subnet (by bits)
pub struct MaxAvailableArgs {}

#[derive(Debug, clap::Subcommand)]
pub enum SubgCommands {
    Allocate(AllocateArgs),
    Cidrs(CidrsArgs),
    Claim(ClaimArgs),
    Free(FreeArgs),
    Init(init::InitArgs),
    MaxAvailable(MaxAvailableArgs),
    Names(NamesArgs),
    Rename(RenameArgs),
}

#[derive(Debug, clap::Args)]
/// Subnet garden command line interface
pub struct SubgArgs {
    #[arg(short = 'p', long, default_value = DEFAULT_STORAGE_PATH, env = "SUBG_POOL_PATH")]
    pub pool_path: String,
}

#[derive(Debug, clap::Parser)]
#[command(
    name = SUBG_COMMAND,
    version = clap::crate_version!(),
    author = clap::crate_authors!(),
)]
pub struct Subg {
    #[command(flatten)]
    pub args: SubgArgs,

    #[command(subcommand)]
    pub command: SubgCommands,
}
