use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short = 'f', long = "from")]
    pub currency: String,

    #[arg(short, long, default_value_t = 1)]
    pub amount: u32,
}
