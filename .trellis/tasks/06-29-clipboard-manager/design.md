# 技术设计: Rust Tauri Windows 剪切板管理工具

## 架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                      Tauri 应用                              │
├─────────────────────────────────────────────────────────────┤
│  Frontend (React + TypeScript)                              │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │   主界面     │ │   搜索      │ │   设置      │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
├─────────────────────────────────────────────────────────────┤
│  Backend (Rust)                                             │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐           │
│  │ 剪切板监控   │ │  数据库     │ │  系统托盘   │           │
│  └─────────────┘ └─────────────┘ └─────────────┘           │
├─────────────────────────────────────────────────────────────┤
│  SQLite 数据库 (可放在 OneDrive 同步目录)                     │
└─────────────────────────────────────────────────────────────┘
```

## 核心模块设计

### 1. 剪切板监控模块 (clipboard_monitor)

**职责**: 监控 Windows 剪切板变化，提取内容并通知其他模块

**实现方案**:
- 使用 Windows API `AddClipboardFormatListener` 监听剪切板变化
- 在独立线程中运行，避免阻塞主线程
- 使用 channel 将剪切板事件发送到主线程

**数据结构**:
```rust
#[derive(Debug, Clone)]
pub struct ClipboardItem {
    pub id: String,           // UUID v7
    pub content_type: ContentType,
    pub content: String,      // 文本内容或文件路径
    pub preview: String,      // 预览文本（截断）
    pub created_at: i64,      // 时间戳
    pub is_favorite: bool,
    pub device_id: String,    // 设备标识
}

#[derive(Debug, Clone)]
pub enum ContentType {
    Text,
    RichText,
    Image,  // 存储文件路径
    Files,  // 文件路径列表
}
```

### 2. 数据库模块 (database)

**职责**: 管理 SQLite 数据库，提供 CRUD 操作

**表结构**:
```sql
CREATE TABLE clipboard_items (
    id TEXT PRIMARY KEY,           -- UUID v7
    content_type TEXT NOT NULL,    -- text/richtext/image/files
    content TEXT NOT NULL,         -- 内容或文件路径
    preview TEXT,                  -- 预览文本
    created_at INTEGER NOT NULL,   -- Unix 时间戳
    is_favorite INTEGER DEFAULT 0, -- 是否收藏
    device_id TEXT NOT NULL,       -- 设备标识
    is_deleted INTEGER DEFAULT 0   -- 软删除标记
);

-- 索引优化查询
CREATE INDEX idx_created_at ON clipboard_items(created_at);
CREATE INDEX idx_content_type ON clipboard_items(content_type);
CREATE INDEX idx_is_favorite ON clipboard_items(is_favorite);
CREATE INDEX idx_is_deleted ON clipboard_items(is_deleted);

-- 全文搜索
CREATE VIRTUAL TABLE clipboard_fts USING fts5(
    content,
    preview,
    content='clipboard_items',
    content_rowid='rowid'
);
```

**同步考虑**:
- 使用 UUID v7 作为主键，多设备生成的 ID 不会冲突
- 使用软删除（`is_deleted`），避免同步时删除操作冲突
- 每条记录包含 `device_id`，便于识别来源设备

### 3. 搜索模块 (search)

**职责**: 提供全文搜索和筛选功能

**实现方案**:
- 使用 SQLite FTS5 全文搜索引擎
- 支持按内容类型、时间范围、收藏状态筛选
- 搜索结果按相关度和时间排序

**查询示例**:
```rust
-- 全文搜索
SELECT * FROM clipboard_items
WHERE id IN (
    SELECT rowid FROM clipboard_fts
    WHERE clipboard_fts MATCH ?
)
AND is_deleted = 0
ORDER BY created_at DESC
LIMIT ? OFFSET ?;

-- 按类型筛选
SELECT * FROM clipboard_items
WHERE content_type = ?
AND is_deleted = 0
ORDER BY created_at DESC;
```

### 4. 系统托盘模块 (tray)

**职责**: 管理系统托盘图标和菜单

**菜单结构**:
```
├── 显示主窗口
├── 最近记录（子菜单）
│   ├── 记录 1（点击复制）
│   ├── 记录 2
│   ├── 记录 3
│   ├── 记录 4
│   └── 记录 5
├── 设置
└── 退出
```

### 5. 快捷键模块 (hotkey)

**职责**: 注册和处理全局快捷键

**实现方案**:
- 使用 Windows API `RegisterHotKey` 注册全局快捷键
- 默认 `Ctrl+W`，可在设置中自定义
- 快捷键触发时显示/隐藏主窗口

## 数据流

```
剪切板变化
    ↓
ClipboardMonitor 监听
    ↓
提取内容 + 生成 UUID v7
    ↓
Database.insert()
    ↓
SQLite 写入（自动同步到 OneDrive）
    ↓
Frontend 更新 UI（通过 Tauri 事件）
```

## 同步策略

### 冲突处理
- UUID v7 基于时间戳，即使多设备同时复制，ID 也不会冲突
- 使用软删除，删除操作不会导致同步冲突
- 最后写入者胜出（对于同一设备的多次更新）

### 性能优化
- 使用 WAL 模式提高并发读写性能
- 批量插入时使用事务
- 定期清理已删除记录（可配置保留天数）

### 数据库锁定
- 使用 `journal_mode=WAL` 避免写入阻塞读取
- 设置合理的超时时间（`busy_timeout`）
- 应用启动时检查数据库是否被其他实例锁定

## 前端设计

### 页面结构
```
├── 主窗口
│   ├── 搜索栏（顶部）
│   ├── 筛选器（类型、时间、收藏）
│   ├── 记录列表（主体）
│   │   ├── 记录卡片
│   │   │   ├── 预览文本
│   │   │   ├── 内容类型图标
│   │   │   ├── 时间
│   │   │   └── 操作按钮（复制、收藏、删除）
│   │   └── 分页/无限滚动
│   └── 底部状态栏
├── 设置窗口
│   ├── 常规设置
│   │   ├── 开机自启
│   │   ├── 记录保留数量
│   │   └── 数据库路径
│   ├── 快捷键设置
│   └── 关于
└── 系统托盘菜单
```

### UI/UX 要求
- 简洁现代的设计风格
- 深色/浅色主题支持
- 响应式布局，支持调整窗口大小
- 流畅的动画效果

## 依赖库

### Rust 后端
```toml
[dependencies]
tauri = { version = "1", features = ["system-tray", "global-shortcut"] }
rusqlite = { version = "0.31", features = ["bundled"] }
uuid = { version = "1", features = ["v7"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"
```

### React 前端
```json
{
  "dependencies": {
    "react": "^18",
    "react-dom": "^18",
    "@tauri-apps/api": "^1",
    "tailwindcss": "^3"
  }
}
```

## 安全考虑

- 不存储敏感信息（无加密）
- 文件路径访问需验证权限
- 防止 SQL 注入（使用参数化查询）
- 限制数据库文件大小（可配置）

## 测试策略

- 单元测试：各模块核心逻辑
- 集成测试：Tauri 命令调用
- E2E 测试：用户操作流程
- 性能测试：大量记录下的查询性能

## 未来扩展

- 支持更多数据格式（HTML、Markdown）
- 云端同步（需网络模块）
- 跨平台支持（macOS、Linux）
- 插件系统
