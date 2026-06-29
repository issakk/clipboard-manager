import { useState, useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useClipboard, useSettings } from './hooks/useClipboard';
import ClipboardList from './components/ClipboardList';
import SearchBar from './components/SearchBar';
import SettingsPanel from './components/SettingsPanel';

function App() {
  const {
    items,
    loading,
    search,
    deleteItem,
    toggleFavorite,
    clearHistory,
  } = useClipboard();

  const { settings, updateSettings } = useSettings();

  const [showSettings, setShowSettings] = useState(false);
  const [copySuccess, setCopySuccess] = useState(false);

  // 监听显示设置事件
  useEffect(() => {
    const unlisten = listen('show-settings', () => {
      setShowSettings(true);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  // 处理搜索
  const handleSearch = (query: string) => {
    search({ query: query || undefined });
  };

  // 处理复制
  const handleCopy = (_content: string) => {
    setCopySuccess(true);
    setTimeout(() => setCopySuccess(false), 2000);
  };

  return (
    <div className="min-h-screen bg-gray-100 dark:bg-gray-900">
      {/* 复制成功提示 */}
      {copySuccess && (
        <div className="fixed top-4 right-4 z-50 bg-green-500 text-white px-4 py-2 rounded-lg shadow-lg animate-fade-in-out">
          ✓ 已复制到剪切板
        </div>
      )}

      {/* 顶部搜索栏 */}
      <header className="bg-white dark:bg-gray-800 shadow-sm sticky top-0 z-10">
        <div className="max-w-7xl mx-auto px-4 py-3">
          <div className="flex items-center justify-between">
            <div className="flex items-center space-x-3">
              <div className="w-8 h-8 bg-blue-500 rounded-lg flex items-center justify-center">
                <svg
                  className="w-5 h-5 text-white"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2"
                  />
                </svg>
              </div>
              <h1 className="text-xl font-semibold text-gray-900 dark:text-white">
                剪切板管理器
              </h1>
            </div>
            <div className="flex items-center space-x-4">
              <span className="text-sm text-gray-500 dark:text-gray-400">
                {items.length} 条记录
              </span>
              <button
                onClick={() => setShowSettings(!showSettings)}
                className="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 rounded-lg hover:bg-gray-100 dark:hover:bg-gray-700"
              >
                <svg
                  className="w-5 h-5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                  />
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                  />
                </svg>
              </button>
              <button
                onClick={clearHistory}
                className="px-3 py-1.5 text-sm text-red-600 hover:text-red-700 hover:bg-red-50 dark:hover:bg-red-900/20 rounded-md transition-colors"
              >
                清空历史
              </button>
            </div>
          </div>
          <div className="mt-3">
            <SearchBar onSearch={handleSearch} />
          </div>
        </div>
      </header>

      {/* 主内容区 */}
      <main className="max-w-7xl mx-auto px-4 py-6">
        {showSettings ? (
          <SettingsPanel
            settings={settings}
            onUpdate={updateSettings}
            onClose={() => setShowSettings(false)}
          />
        ) : (
          <ClipboardList
            items={items}
            loading={loading}
            onCopy={handleCopy}
            onDelete={deleteItem}
            onToggleFavorite={toggleFavorite}
          />
        )}
      </main>

      {/* 底部状态栏 */}
      <footer className="fixed bottom-0 left-0 right-0 bg-white dark:bg-gray-800 border-t border-gray-200 dark:border-gray-700 py-2 px-4">
        <div className="max-w-7xl mx-auto flex items-center justify-between text-xs text-gray-500 dark:text-gray-400">
          <span>
            快捷键: {settings?.shortcut || 'Ctrl+W'}
          </span>
          <span>
            数据库: {settings?.db_path ? '已配置' : '默认位置'}
          </span>
        </div>
      </footer>
    </div>
  );
}

export default App;
