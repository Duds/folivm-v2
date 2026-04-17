#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code)]

mod commands;
mod export;
mod extensions;
mod git;
mod search;

fn main() {
    #[cfg(not(test))]
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::document_read,
            commands::document_save,
            commands::document_save_version,
            commands::export_docx,
            commands::export_pdf,
            commands::library_list,
            commands::project_create,
            commands::project_open,
            commands::search_query,
            commands::search_replace,
            commands::theme_read,
            commands::versioning_diff,
            commands::versioning_list,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
