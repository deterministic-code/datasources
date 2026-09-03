use rusqlite::{Connection, OpenFlags};

pub fn open(filename: &str) -> rusqlite::Result<Connection> {
    if filename.starts_with("file:") {
        Connection::open_with_flags(
            filename,
            OpenFlags::SQLITE_OPEN_READ_WRITE
                | OpenFlags::SQLITE_OPEN_CREATE
                | OpenFlags::SQLITE_OPEN_URI
                | OpenFlags::SQLITE_OPEN_SHARED_CACHE,
        )
    } else {
        Connection::open(filename)
    }
}
