use std::error::Error;

pub mod args;
pub mod db;
pub mod models;
pub mod routes;
pub mod server;

type CalResult<T> = Result<T, Box<dyn Error>>;
