use clap::Parser;
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub bios: bool,
    #[arg(short, long)]
    pub no_screen: bool,

}