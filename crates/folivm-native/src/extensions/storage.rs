use rusqlite::{params, Connection};
use crate::extensions::error::{ExtensionError, Result};
use std::path::Path;
use std::sync::Mutex;

pub struct StorageBackend {
    conn: Mutex<Connection>,
}

impl StorageBackend {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)
            .map_err(|e| ExtensionError::Storage(e.to_string()))?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS extension_storage (
                extension_id TEXT,
                key TEXT,
                value TEXT,
                PRIMARY KEY (extension_id, key)
            )",
            [],
        ).map_err(|e| ExtensionError::Storage(e.to_string()))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn get(&self, extension_id: &str, key: &str) -> Result<Option<String>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT value FROM extension_storage WHERE extension_id = ?1 AND key = ?2"
        ).map_err(|e| ExtensionError::Storage(e.to_string()))?;

        let mut rows = stmt.query(params![extension_id, key])
            .map_err(|e| ExtensionError::Storage(e.to_string()))?;

        if let Some(row) = rows.next().map_err(|e| ExtensionError::Storage(e.to_string()))? {
            Ok(Some(row.get(0).map_err(|e| ExtensionError::Storage(e.to_string()))?))
        } else {
            Ok(None)
        }
    }

    pub fn set(&self, extension_id: &str, key: &str, value: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO extension_storage (extension_id, key, value) VALUES (?1, ?2, ?3)",
            params![extension_id, key, value],
        ).map_err(|e| ExtensionError::Storage(e.to_string()))?;
        Ok(())
    }

    pub fn delete(&self, extension_id: &str, key: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM extension_storage WHERE extension_id = ?1 AND key = ?2",
            params![extension_id, key],
        ).map_err(|e| ExtensionError::Storage(e.to_string()))?;
        Ok(())
    }
}
