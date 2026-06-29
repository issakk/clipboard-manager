import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Settings } from '../types';

interface SettingsPanelProps {
  settings: Settings | null;
  onUpdate: (settings: Settings) => void;
  onClose: () => void;
}

export default function SettingsPanel({
  settings,
  onUpdate,
  onClose,
}: SettingsPanelProps) {
  const [formData, setFormData] = useState<Settings>({
    max_records: 10000,
    shortcut: 'Ctrl+W',
    auto_start: false,
    db_path: '',
  });
  const [dbPath, setDbPath] = useState('');

  useEffect(() => {
    if (settings) {
      setFormData(settings);
    }
    loadDbPath();
  }, [settings]);

  const loadDbPath = async () => {
    try {
      const path = await invoke<string>('get_db_path');
      setDbPath(path);
      setFormData((prev) => ({ ...prev, db_path: path }));
    } catch (error) {
      console.error('Failed to get db path:', error);
    }
  };

  const handleSelectPath = async () => {
    try {
      const path = await invoke<string>('select_db_path');
      setDbPath(path);
      setFormData((prev) => ({ ...prev, db_path: path }));
    } catch (error) {
      console.error('Failed to select path:', error);
    }
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onUpdate(formData);
  };

  const handleChange = (field: keyof Settings, value: any) => {
    setFormData((prev) => ({
      ...prev,
      [field]: value,
    }));
  };

  return (
    <div className="bg-white dark:bg-gray-800 rounded-lg shadow-sm border border-gray-200 dark:border-gray-700">
      <div className="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <div className="flex items-center justify-between">
          <h2 className="text-lg font-semibold text-gray-900 dark:text-white">
            设置
          </h2>
          <button
            onClick={onClose}
            className="p-1 text-gray-400 hover:text-gray-600 dark:hover:text-gray-200"
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
                d="M6 18L18 6M6 6l12 12"
              />
            </svg>
          </button>
        </div>
      </div>

      <form onSubmit={handleSubmit} className="p-6 space-y-6">
        {/* 最大记录数 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
            最大记录数
          </label>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            设置保留的最大剪切板记录数量
          </p>
          <input
            type="number"
            value={formData.max_records}
            onChange={(e) =>
              handleChange('max_records', parseInt(e.target.value))
            }
            min="100"
            max="100000"
            className="mt-2 block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          />
        </div>

        {/* 全局快捷键 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
            全局快捷键
          </label>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            设置呼出剪切板管理器的快捷键
          </p>
          <input
            type="text"
            value={formData.shortcut}
            onChange={(e) => handleChange('shortcut', e.target.value)}
            placeholder="例如: Ctrl+Shift+V"
            className="mt-2 block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:text-white"
          />
        </div>

        {/* 开机自启 */}
        <div className="flex items-center justify-between">
          <div>
            <label className="text-sm font-medium text-gray-700 dark:text-gray-300">
              开机自启
            </label>
            <p className="text-sm text-gray-500 dark:text-gray-400">
              系统启动时自动运行剪切板管理器
            </p>
          </div>
          <button
            type="button"
            onClick={() => handleChange('auto_start', !formData.auto_start)}
            className={`relative inline-flex h-6 w-11 flex-shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 ${
              formData.auto_start ? 'bg-blue-600' : 'bg-gray-200 dark:bg-gray-700'
            }`}
          >
            <span
              className={`pointer-events-none inline-block h-5 w-5 transform rounded-full bg-white shadow ring-0 transition duration-200 ease-in-out ${
                formData.auto_start ? 'translate-x-5' : 'translate-x-0'
              }`}
            />
          </button>
        </div>

        {/* 数据库路径 */}
        <div>
          <label className="block text-sm font-medium text-gray-700 dark:text-gray-300">
            数据库路径
          </label>
          <p className="mt-1 text-sm text-gray-500 dark:text-gray-400">
            设置 SQLite 数据库文件的存储位置（支持 OneDrive 同步盘）
          </p>
          <div className="mt-2 flex space-x-2">
            <input
              type="text"
              value={dbPath}
              readOnly
              className="flex-1 block w-full px-3 py-2 border border-gray-300 dark:border-gray-600 rounded-md shadow-sm bg-gray-50 dark:bg-gray-600 text-gray-500 dark:text-gray-400"
            />
            <button
              type="button"
              onClick={handleSelectPath}
              className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
            >
              选择路径
            </button>
          </div>
          <p className="mt-2 text-xs text-gray-400 dark:text-gray-500">
            💡 提示：将数据库放在 OneDrive/Dropbox 等同步盘目录，可实现多设备同步
          </p>
        </div>

        {/* 提交按钮 */}
        <div className="flex justify-end space-x-3 pt-4 border-t border-gray-200 dark:border-gray-700">
          <button
            type="button"
            onClick={onClose}
            className="px-4 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 bg-white dark:bg-gray-700 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-gray-600 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            取消
          </button>
          <button
            type="submit"
            className="px-4 py-2 text-sm font-medium text-white bg-blue-600 border border-transparent rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
          >
            保存设置
          </button>
        </div>
      </form>
    </div>
  );
}
