use clap::Parser;

/// CAL-Server - the Server Implementation for CAL: the Simple Calendar
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CalArgs {
    /// Whether or not to delete and refresh the databse before starup
    #[clap(short, long)]
    pub reset_db: bool,
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
    #[clap(short, long, default_value = "127.0.0.1")]
    pub domain: String,
}

pub fn get_args() -> CalArgs {
    CalArgs::parse()
}