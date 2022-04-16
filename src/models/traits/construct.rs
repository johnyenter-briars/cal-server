use rusqlite::Row;

pub trait ConstructableFromSql<T> {
    fn construct(row: &Row) -> Result<Self, Box<dyn std::error::Error>> where Self: std::marker::Sized;
}
