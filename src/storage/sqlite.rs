use anyhow::Result;
use rusqlite::Connection;

#[allow(dead_code)]
pub struct SqliteStore {
    pub conn: Connection,
}

#[allow(dead_code)]
impl SqliteStore {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn migrate(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS players (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                chips INTEGER NOT NULL DEFAULT 0,
                created_at TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS game_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                game_type TEXT NOT NULL,
                bet INTEGER NOT NULL,
                result INTEGER NOT NULL,
                played_at TEXT NOT NULL
            );"
        )?;
        Ok(())
    }
}
