use clap::Parser;

#[derive(Parser, Debug)]
pub struct Ps {
    #[clap(short, long, default_value = "table")]
    pub format: String,
    pub filter: String,
    // #[clap(forbid_empty_values = true, required = true)]
    pub latest: String,
    #[clap(name="no-trunc", provided=false)]
    pub no_trunc: String,
    pub quiet: String,
    pub size: String,
}

pub fn ps(args: Ps)  {
    println!("Hello ps !");
    if args.format == "json" {
        println!("serde_json::to_string(&pids)?");
    } else if args.format == "table" {
        println!("split output message!")
    }

}