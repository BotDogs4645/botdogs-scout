use leptos::*;
use rusqlite::*;

pub async fn get_db() -> Result<Connection, Error> {
    let conn = Connection::open("scouting.db")?;
    Ok(conn)
}




