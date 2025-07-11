// src/main.rs
/*
 * Main executable for BlockchainNFTMarketplaceCoreKitPro
 */

use clap::Parser;
use blockchainnftmarketplacecorekitpro::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTMarketplaceCoreKitPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
