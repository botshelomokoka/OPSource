use clap::{App, Arg, SubCommand};
use crate::wallet::WalletManager;
use crate::layers::LayerManager;

pub fn build_cli() -> App<'static, 'static> {
    App::new("Anya Enterprise CLI")
        .version("1.0")
        .author("Anya Enterprise Contributors")
        .about("Multi-layer Bitcoin wallet management system")
        .subcommand(build_transaction_subcommand())
        .subcommand(build_lightning_subcommand())
        .subcommand(build_rgb_subcommand())
        .subcommand(build_dlc_subcommand())
        .subcommand(build_layer_subcommand())
}

fn build_transaction_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("tx")
        .about("Manage transactions across layers")
        .subcommand(SubCommand::with_name("create")
            .about("Create a new transaction")
            .arg(Arg::with_name("LAYER")
                .help("Specify the layer (btc, ln, rgb, dlc)")
                .required(true))
            .arg(Arg::with_name("PARAMS")
                .help("Transaction parameters")
                .required(true)))
        .subcommand(SubCommand::with_name("status")
            .about("Check transaction status")
            .arg(Arg::with_name("TXID")
                .help("Transaction ID")
                .required(true)))
}

fn build_layer_subcommand() -> App<'static, 'static> {
    SubCommand::with_name("layer")
        .about("Cross-layer operations")
        .subcommand(SubCommand::with_name("bridge")
            .about("Bridge between layers")
            .arg(Arg::with_name("FROM")
                .help("Source layer")
                .required(true))
            .arg(Arg::with_name("TO")
                .help("Destination layer")
                .required(true))
            .arg(Arg::with_name("AMOUNT")
                .help("Amount to bridge")
                .required(true)))
}