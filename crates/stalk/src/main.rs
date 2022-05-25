mod commands;

use clap::Parser;
use {ps::Ps};


#[derive(Parser, Debug)]
struct Opts {
    // global: GlobalOpts,
    #[clap(subcommand)]
    subcmd: Subcommand,
}

#[derive(Parser, Debug)]
struct Subcommand {
    #[clap(flatten)]
    Standard(StandardCmd),
    //  后续实现
    // Common(),
    // Extern(),
}

#[derive(Parser, Debug)]
pub enum StandardCmd {
    Ps(Ps)
}


fn main() {
    let opts = Opts::parse();
    match opts.subcmd {
        Subcommand::Standard(cmd) => match cmd {
            StandardCmd::Ps(ps) => commands::ps::ps(ps)
        }
    }
}