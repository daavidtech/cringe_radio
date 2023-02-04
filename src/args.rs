use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "peacemaker")]
pub struct Args {
    #[clap(long, value_parser)]
    pub token: String,
}