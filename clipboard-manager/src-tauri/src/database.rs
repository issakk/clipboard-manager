use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipboardItem {
    pub id: String,
    pub content_type: String,
    pub content: String,
    pub preview: Option<String>,
    pub created_at: i64,
    pub is_favorite: bool,
    pub device_id: String,
    pub is_deleted: bool,
}

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new_with_path(db_path: &str) -> Result<Self> {
        // 确保目录存在
        if let Some(parent) = std::path::Path::new(db_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let conn = Connection::open(db_path)?;

        // 启用 WAL 模式
        conn.execute_batch("PRAGMA journal_mode=WAL;")?;
        conn.execute_batch("PRAGMA busy_timeout=5000;")?;
        conn.execute_batch("PRAGMA synchronous=NORMAL;")?;

        // 创建表
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS clipboard_items (
                id TEXT PRIMARY KEY,
                content_type TEXT NOT NULL,
                content TEXT NOT NULL,
                preview TEXT,
                created_at INTEGER NOT NULL,
                is_favorite INTEGER DEFAULT 0,
                device_id TEXT NOT NULL,
                is_deleted INTEGER DEFAULT 0
            );

            CREATE INDEX IF NOT EXISTS idx_created_at ON clipboard_items(created_at);
            CREATE INDEX IF NOT EXISTS idx_content_type ON clipboard_items(content_type);
            CREATE INDEX IF NOT EXISTS idx_is_favorite ON clipboard_items(is_favorite);
            CREATE INDEX IF NOT EXISTS idx_is_deleted ON clipboard_items(is_deleted);"
        )?;

        // 创建全文搜索表
        conn.execute_batch(
            "CREATE VIRTUAL TABLE IF NOT EXISTS clipboard_fts USING fts5(
                content,
                preview,
                content='clipboard_items',
                content_rowid='rowid'
            );"
        )?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn insert(&self, item: &ClipboardItem) -> Result<()> {
        let conn = self.conn.lock().unwrap();

        // 插入主表
        conn.execute(
            "INSERT INTO clipboard_items (id, content_type, content, preview, created_at, is_favorite, device_id, is_deleted)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                item.id,
                item.content_type,
                item.content,
                item.preview,
                item.created_at,
                item.is_favorite as i32,
                item.device_id,
                item.is_deleted as i32,
            ],
        )?;

        // 获取 rowid 并更新 FTS 索引
        let rowid: i64 = conn.last_insert_rowid();
        conn.execute(
            "INSERT INTO clipboard_fts (rowid, content, preview)
             VALUES (?1, ?2, ?3)",
            params![rowid, item.content, item.preview],
        )?;

        Ok(())
    }

    pub fn get_history(&self, limit: i64, offset: i64) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, content_type, content, preview, created_at, is_favorite, device_id, is_deleted
             FROM clipboard_items
             WHERE is_deleted = 0
             ORDER BY created_at DESC
             LIMIT ?1 OFFSET ?2"
        )?;

        let items = stmt.query_map(params![limit, offset], |row| {
            Ok(ClipboardItem {
                id: row.get(0)?,
                content_type: row.get(1)?,
                content: row.get(2)?,
                preview: row.get(3)?,
                created_at: row.get(4)?,
                is_favorite: row.get::<_, i32>(5)? != 0,
                device_id: row.get(6)?,
                is_deleted: row.get::<_, i32>(7)? != 0,
            })
        })?.collect::<Result<Vec<_>>>()?;

        Ok(items)
    }

    pub fn search(&self, query: &str, content_type: Option<&str>, limit: i64, offset: i64) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();

        let sql = if content_type.is_some() {
            "SELECT c.id, c.content_type, c.content, c.preview, c.created_at, c.is_favorite, c.device_id, c.is_deleted
             FROM clipboard_items c
             INNER JOIN clipboard_fts f ON c.rowid = f.rowid
             WHERE clipboard_fts MATCH ?1 AND c.content_type = ?2 AND c.is_deleted = 0
             ORDER BY c.created_at DESC
             LIMIT ?3 OFFSET ?4"
        } else {
            "SELECT c.id, c.content_type, c.content, c.preview, c.created_at, c.is_favorite, c.device_id, c.is_deleted
             FROM clipboard_items c
             INNER JOIN clipboard_fts f ON c.rowid = f.rowid
             WHERE clipboard_fts MATCH ?1 AND c.is_deleted = 0
             ORDER BY c.created_at DESC
             LIMIT ?2 OFFSET ?3"
        };

        let mut stmt = conn.prepare(sql)?;

        let items = if let Some(ct) = content_type {
            stmt.query_map(params![query, ct, limit, offset], |row| {
                Ok(ClipboardItem {
                    id: row.get(0)?,
                    content_type: row.get(1)?,
                    content: row.get(2)?,
                    preview: row.get(3)?,
                    created_at: row.get(4)?,
                    is_favorite: row.get::<_, i32>(5)? != 0,
                    device_id: row.get(6)?,
                    is_deleted: row.get::<_, i32>(7)? != 0,
                })
            })?.collect::<Result<Vec<_>>>()?
        } else {
            stmt.query_map(params![query, limit, offset], |row| {
                Ok(ClipboardItem {
                    id: row.get(0)?,
                    content_type: row.get(1)?,
                    content: row.get(2)?,
                    preview: row.get(3)?,
                    created_at: row.get(4)?,
                    is_favorite: row.get::<_, i32>(5)? != 0,
                    device_id: row.get(6)?,
                    is_deleted: row.get::<_, i32>(7)? != 0,
                })
            })?.collect::<Result<Vec<_>>>()?
        };

        Ok(items)
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "UPDATE clipboard_items SET is_deleted = 1 WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }

    pub fn toggle_favorite(&self, id: &str) -> Result<bool> {
        let conn = self.conn.lock().unwrap();
        let current: bool = conn.query_row(
            "SELECT is_favorite FROM clipboard_items WHERE id = ?1",
            params![id],
            |row| row.get::<_, i32>(0),
        )? != 0;

        let new_value = !current;
        conn.execute(
            "UPDATE clipboard_items SET is_favorite = ?1 WHERE id = ?2",
            params![new_value as i32, id],
        )?;

        Ok(new_value)
    }

    pub fn clear_history(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("UPDATE clipboard_items SET is_deleted = 1", [])?;
        Ok(())
    }

    pub fn get_recent(&self, limit: i64) -> Result<Vec<ClipboardItem>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT id, content_type, content, preview, created_at, is_favorite, device_id, is_deleted
             FROM clipboard_items
             WHERE is_deleted = 0
             ORDER BY created_at DESC
             LIMIT ?1"
        )?;

        let items = stmt.query_map(params![limit], |row| {
            Ok(ClipboardItem {
                id: row.get(0)?,
                content_type: row.get(1)?,
                content: row.get(2)?,
                preview: row.get(3)?,
                created_at: row.get(4)?,
                is_favorite: row.get::<_, i32>(5)? != 0,
                device_id: row.get(6)?,
                is_deleted: row.get::<_, i32>(7)? != 0,
            })
        })?.collect::<Result<Vec<_>>>()?;

        Ok(items)
    }
}
