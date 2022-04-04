use rusqlite::Row;

pub trait ConstructableFromSql<T> {
    fn construct(row: &Row) -> Self;
}
