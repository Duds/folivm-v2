// Full-text search and replace engine backed by ripgrep.
//
// Responsibilities:
// - Project-wide search across `.fvm` files
// - Regex and plain-text query modes
// - Search-and-replace with preview
// - Streaming results via Tauri events (`search:result`)
//
// Search operates on raw `.fvm` text. Results include file path, line number,
// column, and a snippet for display in the shell's search panel.
