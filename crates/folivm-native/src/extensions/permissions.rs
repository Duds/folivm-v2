use crate::extensions::error::{ExtensionError, Result};
use crate::extensions::manifest::Permissions;

pub struct PermissionGuard {
    permissions: Permissions,
}

impl PermissionGuard {
    pub fn new(permissions: Permissions) -> Self {
        Self { permissions }
    }

    pub fn check_document_read(&self) -> Result<()> {
        if self.permissions.document_read {
            Ok(())
        } else {
            Err(ExtensionError::PermissionDenied("document.read required".into()))
        }
    }

    pub fn check_document_write(&self) -> Result<()> {
        if self.permissions.document_write {
            Ok(())
        } else {
            Err(ExtensionError::PermissionDenied("document.write required".into()))
        }
    }

    pub fn check_cell_render(&self, cell_type: &str) -> Result<()> {
        if self.permissions.cell_render.iter().any(|s| s == cell_type) {
            Ok(())
        } else {
            Err(ExtensionError::PermissionDenied(format!("cell.render:{} required", cell_type)))
        }
    }

    pub fn check_cell_export(&self, cell_type: &str) -> Result<()> {
        if self.permissions.cell_export.iter().any(|s| s == cell_type) {
            Ok(())
        } else {
            Err(ExtensionError::PermissionDenied(format!("cell.export:{} required", cell_type)))
        }
    }

    pub fn check_data_source(&self) -> Result<()> {
        if self.permissions.data_source {
            Ok(())
        } else {
            Err(ExtensionError::PermissionDenied("data.source required".into()))
        }
    }

    pub fn check_panel(&self) -> Result<()> {
        if self.permissions.panel {
            Ok(())
        } else {
            Err(ExtensionError::PermissionDenied("panel required".into()))
        }
    }

    pub fn check_export_hook(&self) -> Result<()> {
        if self.permissions.export_hook {
            Ok(())
        } else {
            Err(ExtensionError::PermissionDenied("export.hook required".into()))
        }
    }

    pub fn check_library_contribute(&self) -> Result<()> {
        if self.permissions.library_contribute {
            Ok(())
        } else {
            Err(ExtensionError::PermissionDenied("library.contribute required".into()))
        }
    }

    pub fn check_network(&self, url: &str) -> Result<()> {
        if self.permissions.network.iter().any(|pattern| url.starts_with(pattern)) {
            Ok(())
        } else {
            Err(ExtensionError::PermissionDenied(format!("network:{} required", url)))
        }
    }
}
