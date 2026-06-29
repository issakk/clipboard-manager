# 实现计划: Rust Tauri Windows 剪切板管理工具

## 阶段 1: 项目初始化

### 1.1 创建 Tauri 项目
```bash
# 使用 create-tauri-app 创建项目
npm create tauri-app@latest clipboard-manager -- --template react-ts
cd clipboard-manager
```

### 1.2 配置 Rust 依赖
```toml
# src-tauri/Cargo.toml
[dependencies]
tauri = { version = "1", features = ["system-tray", "global-shortcut"] }
rusqlite = { version = "0.31", features = ["bundled"] }
uuid = { version = "1", features = ["v7"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"
```

### 1.3 配置前端依赖
```bash
npm install @tauri-apps/api tailwindcss postcss autoprefixer
npx tailwindcss init -p
```

### 1.4 项目结构
```
clipboard-manager/
├── src/                    # React 前端
│   ├── components/
│   │   ├── ClipboardList.tsx
│   │   ├── SearchBar.tsx
│   │   ├── Settings.tsx
│   │   └── TrayMenu.tsx
│   ├── hooks/
│   │   └── useClipboard.ts
│   ├── types/
│   │   └── index.ts
│   ├── App.tsx
│   └── main.tsx
├── src-tauri/              # Rust 后端
│   ├── src/
│   │   ├── main.rs
│   │   ├── clipboard_monitor.rs
│   │   ├── database.rs
│   │   ├── search.rs
│   │   ├── tray.rs
│   │   ├── hotkey.rs
│   │   └── commands.rs
│   ├── Cargo.toml
│   └── tauri.conf.json
└── package.json
```

## 阶段 2: 后端核心实现

### 2.1 数据库模块 (database.rs)
- [ ] 创建 SQLite 连接和表结构
- [ ] 实现插入、查询、更新、删除操作
- [ ] 实现软删除机制
- [ ] 配置 WAL 模式和忙等待超时
- [ ] 实现批量插入优化

### 2.2 剪切板监控模块 (clipboard_monitor.rs)
- [ ] 实现 Windows 剪切板监听
- [ ] 提取文本、图片、文件路径
- [ ] 生成 UUID v7
- [ ] 发送事件到主线程

### 2.3 搜索模块 (search.rs)
- [ ] 实现 FTS5 全文搜索
- [ ] 实现按类型、时间筛选
- [ ] 优化搜索性能

### 2.4 系统托盘模块 (tray.rs)
- [ ] 创建托盘图标
- [ ] 实现右键菜单
- [ ] 实现最近记录子菜单

### 2.5 快捷键模块 (hotkey.rs)
- [ ] 注册全局快捷键
- [ ] 实现快捷键触发逻辑
- [ ] 支持自定义快捷键

### 2.6 Tauri 命令 (commands.rs)
- [ ] 实现前后端通信接口
- [ ] 暴露数据库操作给前端
- [ ] 实现事件推送机制

## 阶段 3: 前端实现

### 3.1 主界面组件
- [ ] ClipboardList 组件（记录列表）
- [ ] SearchBar 组件（搜索栏）
- [ ] FilterBar 组件（筛选器）
- [ ] ClipboardCard 组件（记录卡片）

### 3.2 设置界面
- [ ] Settings 组件
- [ ] 常规设置（开机自启、记录数量）
- [ ] 快捷键设置
- [ ] 数据库路径设置

### 3.3 状态管理
- [ ] 实现 useClipboard Hook
- [ ] 实现 Tauri 事件监听
- [ ] 实现本地状态管理

### 3.4 样式和主题
- [ ] 配置 Tailwind CSS
- [ ] 实现深色/浅色主题
- [ ] 响应式布局

## 阶段 4: 功能集成

### 4.1 剪切板监控集成
- [ ] 启动时开始监控
- [ ] 监控到变化时更新数据库
- [ ] 推送事件到前端

### 4.2 搜索功能集成
- [ ] 实现实时搜索
- [ ] 实现筛选功能
- [ ] 优化搜索体验

### 4.3 系统托盘集成
- [ ] 实现托盘菜单功能
- [ ] 实现最近记录快速粘贴
- [ ] 实现窗口显示/隐藏

### 4.4 快捷键集成
- [ ] 注册默认快捷键
- [ ] 实现快捷键触发逻辑
- [ ] 支持自定义快捷键

## 阶段 5: 测试和优化

### 5.1 单元测试
- [ ] 数据库模块测试
- [ ] 搜索模块测试
- [ ] 剪切板监控测试

### 5.2 集成测试
- [ ] Tauri 命令测试
- [ ] 前后端通信测试
- [ ] 系统托盘测试

### 5.3 性能优化
- [ ] 优化数据库查询
- [ ] 优化内存使用
- [ ] 优化 CPU 占用

### 5.4 用户体验优化
- [ ] 优化启动速度
- [ ] 优化搜索响应
- [ ] 优化动画效果

## 阶段 6: 打包和发布

### 6.1 配置打包
- [ ] 配置 tauri.conf.json
- [ ] 设置应用图标
- [ ] 配置安装程序

### 6.2 构建和测试
- [ ] 构建 Windows 安装包
- [ ] 测试安装和卸载
- [ ] 测试各功能模块

### 6.3 文档和发布
- [ ] 编写用户文档
- [ ] 编写开发者文档
- [ ] 发布到 GitHub

## 验证命令

```bash
# 开发环境运行
npm run tauri dev

# 构建生产版本
npm run tauri build

# 运行测试
cargo test
npm test

# 代码检查
cargo clippy
npm run lint
```

## 风险和注意事项

1. **Windows API 兼容性**: 确保在不同 Windows 版本上正常工作
2. **数据库锁定**: 正确处理 SQLite 锁，避免同步盘冲突
3. **性能监控**: 监控 CPU 和内存使用，确保符合要求
4. **快捷键冲突**: 测试默认快捷键是否与其他应用冲突
5. **同步盘兼容性**: 测试 OneDrive、Dropbox 等同步盘的兼容性

## 时间估算

- 阶段 1: 1 天
- 阶段 2: 3 天
- 阶段 3: 2 天
- 阶段 4: 2 天
- 阶段 5: 2 天
- 阶段 6: 1 天

**总计**: 约 11 天

## 依赖项

- Node.js 18+
- Rust 1.70+
- Windows 10/11
- Visual Studio Build Tools
