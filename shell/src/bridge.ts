import { invoke } from '@tauri-apps/api/core'

// ---------------------------------------------------------------------------
// Shared response types (mirrors crates/folivm-native/src/commands/mod.rs)
// ---------------------------------------------------------------------------

export interface ProjectMeta {
  name: string
  path: string
}

export interface VersionEntry {
  id: string
  message: string
  timestamp: number
}

export interface SearchResult {
  path: string
  line: number
  column: number
  snippet: string
}

export interface ReplaceResult {
  files_changed: number
  replacements: number
}

export interface LibraryItem {
  id: string
  name: string
  kind: string
}

export interface TrackChangesDiff {
  from: string
  to: string
  hunks: unknown[]
}

// ---------------------------------------------------------------------------
// Project
// ---------------------------------------------------------------------------

export const projectOpen = (path: string): Promise<ProjectMeta> =>
  invoke('project_open', { path })

export const projectCreate = (name: string, path: string): Promise<ProjectMeta> =>
  invoke('project_create', { name, path })

// ---------------------------------------------------------------------------
// Document
// ---------------------------------------------------------------------------

/** Returns the raw `.fvm` string. Parsing happens in folivm-core (WASM). */
export const documentRead = (path: string): Promise<string> =>
  invoke('document_read', { path })

export const documentSave = (path: string, content: string): Promise<void> =>
  invoke('document_save', { path, content })

export const documentSaveVersion = (path: string, message: string): Promise<VersionEntry> =>
  invoke('document_save_version', { path, message })

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

/** Returns the raw `.fvm-theme` string. Parsing happens in folivm-core (WASM). */
export const themeRead = (path: string): Promise<string> =>
  invoke('theme_read', { path })

// ---------------------------------------------------------------------------
// Search
// ---------------------------------------------------------------------------

export const searchQuery = (
  projectPath: string,
  query: string,
  opts: Record<string, unknown>,
): Promise<SearchResult[]> =>
  invoke('search_query', { project_path: projectPath, query, opts })

export const searchReplace = (
  projectPath: string,
  query: string,
  replacement: string,
  opts: Record<string, unknown>,
): Promise<ReplaceResult> =>
  invoke('search_replace', { project_path: projectPath, query, replacement, opts })

// ---------------------------------------------------------------------------
// Export
// ---------------------------------------------------------------------------

export const exportPdf = (
  docPath: string,
  opts: Record<string, unknown>,
): Promise<string> =>
  invoke('export_pdf', { doc_path: docPath, opts })

export const exportDocx = (
  docPath: string,
  opts: Record<string, unknown>,
): Promise<string> =>
  invoke('export_docx', { doc_path: docPath, opts })

// ---------------------------------------------------------------------------
// Versioning
// ---------------------------------------------------------------------------

export const versioningList = (projectPath: string): Promise<VersionEntry[]> =>
  invoke('versioning_list', { project_path: projectPath })

export const versioningDiff = (
  projectPath: string,
  from: string,
  to: string,
): Promise<TrackChangesDiff> =>
  invoke('versioning_diff', { project_path: projectPath, from, to })

// ---------------------------------------------------------------------------
// Library
// ---------------------------------------------------------------------------

export const libraryList = (projectPath: string): Promise<LibraryItem[]> =>
  invoke('library_list', { project_path: projectPath })
