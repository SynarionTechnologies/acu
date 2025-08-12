use super::{KvError, KvStore};
use rusqlite::{params, Connection};
use std::path::Path;
use std::sync::Mutex;

/// SQLite implementation of [`KvStore`].
pub struct SqliteStore {
    conn: Mutex<Connection>,
}

impl SqliteStore {
    /// Open a new store at the given path.
    pub fn new(path: impl AsRef<Path>) -> Result<Self, KvError> {
        let conn = Connection::open(path)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    fn ensure_table(&self, table: &str) -> Result<(), KvError> {
        let sql = format!("CREATE TABLE IF NOT EXISTS kv_{table} (k TEXT PRIMARY KEY, v BLOB)");
        let conn = self.conn.lock().unwrap();
        conn.execute(&sql, [])?;
        Ok(())
    }
}

impl KvStore for SqliteStore {
    fn put(&self, table: &str, key: &str, value: &[u8]) -> Result<(), KvError> {
        self.ensure_table(table)?;
        let sql = format!("INSERT OR REPLACE INTO kv_{table} (k,v) VALUES (?1,?2)");
        let conn = self.conn.lock().unwrap();
        conn.execute(&sql, params![key, value])?;
        Ok(())
    }

    fn get(&self, table: &str, key: &str) -> Result<Option<Vec<u8>>, KvError> {
        self.ensure_table(table)?;
        let sql = format!("SELECT v FROM kv_{table} WHERE k=?1");
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&sql)?;
        let mut rows = stmt.query(params![key])?;
        if let Some(row) = rows.next()? {
            let v: Vec<u8> = row.get(0)?;
            Ok(Some(v))
        } else {
            Ok(None)
        }
    }

    fn delete(&self, table: &str, key: &str) -> Result<(), KvError> {
        self.ensure_table(table)?;
        let sql = format!("DELETE FROM kv_{table} WHERE k=?1");
        let conn = self.conn.lock().unwrap();
        conn.execute(&sql, params![key])?;
        Ok(())
    }

    fn scan_prefix(&self, table: &str, prefix: &str) -> Result<Vec<(String, Vec<u8>)>, KvError> {
        self.ensure_table(table)?;
        let like = format!("{prefix}%");
        let sql = format!("SELECT k,v FROM kv_{table} WHERE k LIKE ?1 ORDER BY k");
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(&sql)?;
        let rows = stmt.query_map(params![like], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, Vec<u8>>(1)?))
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }
}
