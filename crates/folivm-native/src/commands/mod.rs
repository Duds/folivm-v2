use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Shared response types
// ---------------------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectMeta {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VersionEntry {
    pub id: String,
    pub message: String,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub path: String,
    pub line: u32,
    pub column: u32,
    pub snippet: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReplaceResult {
    pub files_changed: u32,
    pub replacements: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LibraryItem {
    pub id: String,
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackChangesDiff {
    pub from: String,
    pub to: String,
    pub hunks: Vec<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// Project commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn project_open(_path: String) -> Result<ProjectMeta, String> {
    todo!()
}

#[tauri::command]
pub async fn project_create(_name: String, _path: String) -> Result<ProjectMeta, String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Document commands
// ---------------------------------------------------------------------------

/// Returns the raw `.fvm` string. Parsing happens in folivm-core, never here.
/// The backend never interprets document content.
#[tauri::command]
pub async fn document_read(_path: String) -> Result<String, String> {
    todo!()
}

/// Writes `content` to `path`. The caller is responsible for serialisation.
/// Writes are atomic: content is written to a temp file and renamed on success.
#[tauri::command]
pub async fn document_save(_path: String, _content: String) -> Result<(), String> {
    todo!()
}

#[tauri::command]
pub async fn document_save_version(_path: String, _message: String) -> Result<VersionEntry, String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Theme commands
// ---------------------------------------------------------------------------

/// Returns the raw `.fvm-theme` string. Parsing happens in folivm-core.
#[tauri::command]
pub async fn theme_read(_path: String) -> Result<String, String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Search commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn search_query(
    _project_path: String,
    _query: String,
    _opts: serde_json::Value,
) -> Result<Vec<SearchResult>, String> {
    todo!()
}

#[tauri::command]
pub async fn search_replace(
    _project_path: String,
    _query: String,
    _replacement: String,
    _opts: serde_json::Value,
) -> Result<ReplaceResult, String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Export commands
// ---------------------------------------------------------------------------

/// Exports to PDF/UA. Writes to a temp file and renames on success so a failed
/// export never corrupts the previous output.
#[tauri::command]
pub async fn export_pdf(_doc_path: String, _opts: serde_json::Value) -> Result<String, String> {
    todo!()
}

/// Exports to DOCX. Writes to a temp file and renames on success.
#[tauri::command]
pub async fn export_docx(_doc_path: String, _opts: serde_json::Value) -> Result<String, String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Versioning commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn versioning_list(_project_path: String) -> Result<Vec<VersionEntry>, String> {
    todo!()
}

#[tauri::command]
pub async fn versioning_diff(
    _project_path: String,
    _from: String,
    _to: String,
) -> Result<TrackChangesDiff, String> {
    todo!()
}

// ---------------------------------------------------------------------------
// Library commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub async fn library_list(_project_path: String) -> Result<Vec<LibraryItem>, String> {
    todo!()
}
