use std::borrow::{Borrow, BorrowMut};

use anyhow::Result;
use clap::{arg, Parser};


#[derive(Parser, Debug)]
pub struct Ps {
    #[clap(short='f', long, default_value = "table")]
    pub format: String,
    #[clap(long)]
    pub filter: String,
    // // #[clap(forbid_empty_values = true, required = true)]
    // #[clap(long, short = 'l')]
    // pub latest: String,
    // #[clap(long)]
    // pub no_trunc: String,
    // #[clap(long, short = 'q')]
    // pub quiet: String,
    // #[clap(long, short = 's')]
    // pub size: String,
}

fn check_flag(cmd: &Ps) -> Result<()> {
    Ok(())
}

pub fn ps(args: Ps) {
    println!("Hello ps !");
    // check Flags
    let  err = check_flag(&args);
    match err {
        Ok(_) => println!("flag check ok~~"),
        _ => println!("flag check fail!!")
    };
    let filter = args.filter;
    // let latest = args.borrow().unwarp();
    if args.format == "json" {
        println!("serde_json::to_string(&pids)?");
    } else if args.format == "table" {
        println!("split output message!")
    }
}