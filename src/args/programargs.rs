use clap::Parser;

/// CAL-Server - the Server Implementation for CAL: the Simple Calendar
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct CalArgs {
    /// Whether or not to delete and refresh the database before starup
    #[clap(short, long)]
    pub reset_db: bool,
    /// Populate the database with test data
    #[clap(short, long)]
    pub test_data: bool,
    /// Port for the application should bind to
    #[clap(short, long, default_value_t = 8080)]
    pub port: u16,
    /// Ip Address the application should bind to
    #[clap(short, long, default_value = "127.0.0.1")]
    pub ip: String,
}

pub fn get_args() -> CalArgs {
    CalArgs::parse()
}