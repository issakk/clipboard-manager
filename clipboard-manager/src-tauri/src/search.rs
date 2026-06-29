use crate::database::{ClipboardItem, Database};
use rusqlite::Result;

pub struct SearchEngine<'a> {
    db: &'a Database,
}

impl<'a> SearchEngine<'a> {
    pub fn new(db: &'a Database) -> Self {
        Self { db }
    }

    pub fn search(
        &self,
        query: &str,
        content_type: Option<&str>,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ClipboardItem>> {
        if query.is_empty() {
            return self.db.get_history(limit, offset);
        }

        // 构建 FTS5 查询
        let fts_query = self.build_fts_query(query);
        self.db.search(&fts_query, content_type, limit, offset)
    }

    fn build_fts_query(&self, query: &str) -> String {
        // 处理特殊字符，避免 FTS5 语法错误
        let sanitized = query
            .replace("\"", "\"\"")
            .replace("*", "")
            .replace("?", "");

        // 使用前缀匹配
        let words: Vec<String> = sanitized
            .split_whitespace()
            .map(|word| format!("\"{}\"*", word))
            .collect();

        words.join(" AND ")
    }

    pub fn search_by_type(
        &self,
        content_type: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ClipboardItem>> {
        self.db.search("", Some(content_type), limit, offset)
    }

    pub fn search_favorites(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<ClipboardItem>> {
        // 这里需要扩展数据库方法来支持收藏筛选
        // 暂时使用通用搜索
        self.db.get_history(limit, offset)
    }
}
