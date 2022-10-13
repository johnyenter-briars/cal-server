use rusqlite::Row;

use crate::CalResult;

pub trait ConstructableFromSql<T> {
    fn construct(row: &Row) -> CalResult<Self>
    where
        Self: std::marker::Sized;
}
