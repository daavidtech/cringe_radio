use clap::Parser;

#[derive(Debug, Parser)]
#[clap(name = "peacemaker")]
pub struct Args {
    #[clap(long, value_parser)]
    pub discord_apikey: Option<String>,
    #[clap(long, value_parser)]
    pub youtube_apikey: Option<String>,
    #[clap(long, value_parser)]
    pub openai_apikey: Option<String>,
}