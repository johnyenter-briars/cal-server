use clap::Parser;

/// CAL-Server - the Server Implementation for CAL: the Simple Calendar
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CalArgs {
    /// Whether or not to delete and refresh the databse before starup
    #[clap(short, long)]
    pub reset_db: bool,
}

pub fn get_args() -> CalArgs {
    let args = CalArgs::parse();
    args
}