mod commands;

use clap::Parser;
use ps::Ps;


#[derive(Parser, Debug)]
struct Opts {
    // global: GlobalOpts,
    #[clap(subcommand)]
    subcmd: Subcommand,
}

// #[derive(Parser, Debug)]
// pub enum StandardCmd {
//     // Create(Create),
//     // Start(Start),
//     // State(State),
//     // Kill(Kill),
//     // Delete(Delete),
// }

use commands::ps;
#[derive(Parser, Debug)]
pub enum CommonCmd {
    Ps(Ps),
}

#[derive(Parser, Debug)]
enum  Subcommand {
    // #[clap(flatten)]
    // Standard(StandardCmd),
    //  后续实现
    #[clap(flatten)]
    Common(CommonCmd),
    // Extern(),
}



fn main() {
    let opts = Opts::parse();
    match opts.subcmd {
        Subcommand::Common(cmd) => match cmd {
            CommonCmd::Ps(ps) =>  commands::ps::ps(ps)
        }
    }
}