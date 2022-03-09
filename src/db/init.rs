use rusqlite::{params, Connection, Result};

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

pub fn initiaize_db() -> Result<bool, Box<dyn std::error::Error>> {
    println!("initingg");
    let conn = Connection::open("cal.db")?;

    // let mut stmt = conn.prepare(
    //     "select name from sqlite_master
    //         where type = 'table';",
    // )?;
    // let table_iter = stmt.query_map([], |row| Ok(row.get::<usize, String>(0)?))?;

    // for table in table_iter {
    //     println!("{:?}", table.unwrap());
    //     // conn.execute(&format!("drop table {};", table.unwrap()), [])?;
    // }

    // drop(stmt);

    println!("we made it here?");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS person (
                  id              INTEGER PRIMARY KEY,
                  name            TEXT NOT NULL,
                  data            BLOB
                  )",
        [],
    )?;

    conn.execute(
        " delete from person
                  ",
        [],
    )?;

    match conn.close() {
        Ok(_) => Ok(true),
        Err((_, err)) => Err(Box::new(err)),
    }
}
