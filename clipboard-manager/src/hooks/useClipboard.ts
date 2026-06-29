import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { ClipboardItem, SearchParams, Settings } from '../types';

export function useClipboard() {
  const [items, setItems] = useState<ClipboardItem[]>([]);
  const [loading, setLoading] = useState(false);
  const [searchParams, setSearchParams] = useState<SearchParams>({
    limit: 50,
    offset: 0,
  });

  // 加载历史记录
  const loadHistory = useCallback(async (limit = 50, offset = 0) => {
    setLoading(true);
    try {
      const result = await invoke<ClipboardItem[]>('get_clipboard_history', {
        limit,
        offset,
      });
      setItems(result);
    } catch (error) {
      console.error('Failed to load history:', error);
    } finally {
      setLoading(false);
    }
  }, []);

  // 搜索剪切板
  const search = useCallback(async (params: SearchParams) => {
    setLoading(true);
    try {
      const result = await invoke<ClipboardItem[]>('search_clipboard', {
        params: {
          ...params,
          limit: params.limit || 50,
          offset: params.offset || 0,
        },
      });
      setItems(result);
      setSearchParams(params);
    } catch (error) {
      console.error('Search failed:', error);
    } finally {
      setLoading(false);
    }
  }, []);

  // 删除记录
  const deleteItem = useCallback(async (id: string) => {
    try {
      await invoke('delete_clipboard_item', { id });
      setItems((prev) => prev.filter((item) => item.id !== id));
    } catch (error) {
      console.error('Failed to delete item:', error);
    }
  }, []);

  // 切换收藏状态
  const toggleFavorite = useCallback(async (id: string) => {
    try {
      const newFavorite = await invoke<boolean>('toggle_favorite', { id });
      setItems((prev) =>
        prev.map((item) =>
          item.id === id ? { ...item, is_favorite: newFavorite } : item
        )
      );
    } catch (error) {
      console.error('Failed to toggle favorite:', error);
    }
  }, []);

  // 清空历史记录
  const clearHistory = useCallback(async () => {
    try {
      await invoke('clear_history');
      setItems([]);
    } catch (error) {
      console.error('Failed to clear history:', error);
    }
  }, []);

  // 复制到剪切板
  const copyToClipboard = useCallback(async (content: string) => {
    try {
      await navigator.clipboard.writeText(content);
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
    }
  }, []);

  // 监听剪切板变化事件
  useEffect(() => {
    const unlisten = listen<ClipboardItem>('clipboard-changed', (event) => {
      const newItem = event.payload;
      setItems((prev) => [newItem, ...prev]);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  // 初始加载
  useEffect(() => {
    loadHistory();
  }, [loadHistory]);

  return {
    items,
    loading,
    searchParams,
    loadHistory,
    search,
    deleteItem,
    toggleFavorite,
    clearHistory,
    copyToClipboard,
  };
}

export function useSettings() {
  const [settings, setSettings] = useState<Settings | null>(null);

  // 加载设置
  const loadSettings = useCallback(async () => {
    try {
      const result = await invoke<Settings>('get_settings');
      setSettings(result);
    } catch (error) {
      console.error('Failed to load settings:', error);
    }
  }, []);

  // 更新设置
  const updateSettings = useCallback(async (newSettings: Settings) => {
    try {
      await invoke('update_settings', { settings: newSettings });
      setSettings(newSettings);
    } catch (error) {
      console.error('Failed to update settings:', error);
    }
  }, []);

  useEffect(() => {
    loadSettings();
  }, [loadSettings]);

  return {
    settings,
    loadSettings,
    updateSettings,
  };
}
