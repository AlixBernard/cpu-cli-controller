#[allow(dead_code)]
#[allow(unused)]
mod commands;
mod utils;

use clap::{Args, Parser, Subcommand};

use commands::{activate_cmd, deactivate_cmd, show_cmd};

#[derive(Parser)]
// #[command(version, about, long_about = None)]
#[command(
    name = "cpu-cli-controller",
    version,
    about = "A program to control/toggle on-off the CPU cores",
    after_long_help = "Bugs can be reported on GitHub: https://github.com/AlixBernard/cpu-cli-controller/issues",
    max_term_width = 98
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args, Debug)]
struct OptionalCoresArgs {
    #[arg(
        short,
        long,
        help = "Specify the cores to consider, eg. '2,3-5,11'",
        required = false,
        value_name = "RANGES"
    )]
    cores: Option<String>,

    #[arg(
        short = 'D',
        long,
        help = "Remove duplicates of cores specified with the option '--cores'",
        required = false,
        action
    )]
    no_duplicates: bool,

    #[arg(
        short,
        long,
        help = "Sort the cores in increasing order",
        required = false,
        action
    )]
    sort: bool,
}

#[derive(Args, Debug)]
struct CoresArgs {
    #[arg(
        short,
        long,
        help = "Specify the cores to consider, eg. '2,3-5,11'",
        required = true,
        value_name = "RANGES"
    )]
    cores: String,

    #[arg(
        short = 'D',
        long,
        help = "Remove duplicates of cores specified with the option '--cores'",
        required = false,
        action
    )]
    no_duplicates: bool,

    #[arg(
        short,
        long,
        help = "Sort the cores in increasing order",
        required = false,
        action
    )]
    sort: bool,
}

// #[derive(Args, Debug)]
// struct DisplayArgs {
//     #[arg(short, long, required = false, action)]
//     no_duplicate: bool,

//     #[arg(short, long, required = false, action)]
//     sort: bool,
// }

#[derive(Subcommand, Debug)]
enum Commands {
    #[clap(visible_alias = "a")]
    Activate(OptionalCoresArgs),

    #[clap(visible_alias = "d")]
    Deactivate(CoresArgs),

    #[clap(visible_alias = "s")]
    Show(OptionalCoresArgs),
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Activate(args) => activate_cmd(args),
        Commands::Deactivate(args) => deactivate_cmd(args),
        Commands::Show(args) => show_cmd(args),
    }
}
