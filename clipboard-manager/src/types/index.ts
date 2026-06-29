export interface ClipboardItem {
  id: string;
  content_type: 'text' | 'richtext' | 'image' | 'files';
  content: string;
  preview: string | null;
  created_at: number;
  is_favorite: boolean;
  device_id: string;
  is_deleted: boolean;
}

export interface SearchParams {
  query?: string;
  content_type?: string;
  limit?: number;
  offset?: number;
}

export interface Settings {
  max_records: number;
  shortcut: string;
  auto_start: boolean;
  db_path: string;
}

export type ContentType = 'text' | 'richtext' | 'image' | 'files';
