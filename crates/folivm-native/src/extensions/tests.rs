#[cfg(test)]
mod tests {
    use crate::extensions::manifest::ExtensionManifest;
    use crate::extensions::permissions::PermissionGuard;
    use crate::extensions::hooks::{HookBus, HookEvent};
    use crate::extensions::registry::ExtensionRegistry;
    use std::sync::Arc;

    #[test]
    fn test_manifest_parsing() {
        let json = r#"{
            "id": "com.example.test",
            "name": "Test Extension",
            "version": "1.0.0",
            "description": "A test extension",
            "min_folivm_version": "1.0.0",
            "entry": "index.js",
            "permissions": {
                "document_read": true,
                "cell_render": ["math"]
            }
        }"#;
        let manifest = ExtensionManifest::from_json(json).unwrap();
        assert_eq!(manifest.id, "com.example.test");
        assert!(manifest.permissions.document_read);
        assert_eq!(manifest.permissions.cell_render, vec!["math"]);
    }

    #[test]
    fn test_permission_guard() {
        let json = r#"{
            "id": "com.example.test",
            "name": "Test Extension",
            "version": "1.0.0",
            "description": "A test extension",
            "min_folivm_version": "1.0.0",
            "entry": "index.js",
            "permissions": {
                "document_read": true,
                "network": ["https://api.example.com"]
            }
        }"#;
        let manifest = ExtensionManifest::from_json(json).unwrap();
        let guard = PermissionGuard::new(manifest.permissions);

        assert!(guard.check_document_read().is_ok());
        assert!(guard.check_document_write().is_err());
        assert!(guard.check_network("https://api.example.com/data").is_ok());
        assert!(guard.check_network("https://malicious.com").is_err());
    }

    #[test]
    fn test_hook_bus() {
        let bus = HookBus::new();
        let received = Arc::new(std::sync::Mutex::new(Vec::new()));
        let received_clone = received.clone();

        bus.subscribe("test-ext".to_string(), Box::new(move |event| {
            received_clone.lock().unwrap().push(event);
            Ok(())
        }));

        let event = HookEvent::ApplicationReady;
        bus.emit(event).unwrap();

        {
            let events = received.lock().unwrap();
            assert_eq!(events.len(), 1);
            if let HookEvent::ApplicationReady = &events[0] {
                // Success
            } else {
                panic!("Expected ApplicationReady event");
            }
        }
    }

    #[test]
    fn test_registry_lifecycle() {
        let hooks = Arc::new(HookBus::new());
        let registry = ExtensionRegistry::new(hooks, std::path::Path::new(":memory:")).unwrap();
        
        let json = r#"{
            "id": "com.example.test",
            "name": "Test Extension",
            "version": "1.0.0",
            "description": "A test extension",
            "min_folivm_version": "1.0.0",
            "entry": "index.js",
            "permissions": {}
        }"#;

        // 1. Initial Load
        let agent = registry.load_manifest(json).unwrap();
        assert_eq!(agent.manifest.id, "com.example.test");
        {
            let state = agent.state.read().unwrap();
            assert_eq!(*state, crate::extensions::agent::ExtensionState::Active);
        }
        
        // 2. Duplicate Load should fail
        let result = registry.load_manifest(json);
        assert!(result.is_err());

        // 3. Retrieval
        let retrieved = registry.get_agent("com.example.test").unwrap();
        assert_eq!(retrieved.manifest.id, "com.example.test");

        // 4. Unload
        registry.unload("com.example.test").unwrap();
        assert!(registry.get_agent("com.example.test").is_none());
        {
            let state = agent.state.read().unwrap();
            assert_eq!(*state, crate::extensions::agent::ExtensionState::Disabled);
        }

        // 5. Unload non-existent should fail
        let result = registry.unload("com.example.test");
        assert!(result.is_err());
    }

    #[test]
    fn test_extension_smoke_bootstrap() {
        let hooks = Arc::new(HookBus::new());
        let registry = ExtensionRegistry::new(hooks, std::path::Path::new(":memory:")).unwrap();
        
        let json = r#"{
            "id": "com.example.smoke",
            "name": "Smoke Test",
            "version": "1.0.0",
            "description": "A smoke test extension",
            "min_folivm_version": "1.0.0",
            "entry": "index.js",
            "permissions": {}
        }"#;

        let agent = registry.load_manifest(json).unwrap();
        let mut runtime_guard = agent.runtime.lock().unwrap();
        let runtime = runtime_guard.as_mut().unwrap();

        // Smoke Test 1: Console Logging
        // This script uses the 'folivm' global defined in bootstrap.js
        let script = r#"
            console.log("Hello from Folivm Extension!");
            console.log("My ID is: " + folivm.extension.id);
            console.log("My Name is: " + folivm.extension.name);
        "#;
        
        runtime.execute_script("smoke:index", script).unwrap();
    }

    #[test]
    fn test_extension_smoke_api() {
        let hooks = Arc::new(HookBus::new());
        let registry = ExtensionRegistry::new(hooks, std::path::Path::new(":memory:")).unwrap();
        
        let json = r#"{
            "id": "com.example.api_test",
            "name": "API Test",
            "version": "1.0.0",
            "description": "An API smoke test",
            "min_folivm_version": "1.0.0",
            "entry": "index.js",
            "permissions": {}
        }"#;

        let agent = registry.load_manifest(json).unwrap();
        let mut runtime_guard = agent.runtime.lock().unwrap();
        let runtime = runtime_guard.as_mut().unwrap();

        // Smoke Test 2: API Surface
        let script = r#"
            const meta = folivm.document.getMetadata();
            console.log("Document Title: " + meta.title);
            
            const blocks = folivm.document.getBlocks();
            console.log("Blocks found: " + blocks.length);
            
            folivm.cells.registerRenderer("math", (cell) => {
                return "rendered math";
            });
        "#;
        
        runtime.execute_script("smoke:api", script).unwrap();
    }

    #[test]
    fn test_extension_smoke_isolation() {
        let hooks = Arc::new(HookBus::new());
        let registry = ExtensionRegistry::new(hooks, std::path::Path::new(":memory:")).unwrap();
        
        let json_a = r#"{
            "id": "com.example.a",
            "name": "Extension A",
            "version": "1.0.0",
            "description": "Ext A",
            "min_folivm_version": "1.0.0",
            "entry": "index.js",
            "permissions": {}
        }"#;
        
        let json_b = r#"{
            "id": "com.example.b",
            "name": "Extension B",
            "version": "1.0.0",
            "description": "Ext B",
            "min_folivm_version": "1.0.0",
            "entry": "index.js",
            "permissions": {}
        }"#;

        let agent_a = registry.load_manifest(json_a).unwrap();
        let agent_b = registry.load_manifest(json_b).unwrap();

        let mut runtime_a = agent_a.runtime.lock().unwrap();
        let mut runtime_b = agent_b.runtime.lock().unwrap();
        
        let rt_a = runtime_a.as_mut().unwrap();
        let rt_b = runtime_b.as_mut().unwrap();

        // Set a global in A
        rt_a.execute_script("a:set", "globalThis.is_a = true;").unwrap();
        
        // B should NOT see it
        let script_b = r#"
            if (globalThis.is_a === undefined) {
                console.log("Extension B is isolated (correct)");
            } else {
                throw new Error("Extension B leaked into A!");
            }
        "#;
        rt_b.execute_script("b:check", script_b).unwrap();
        
        // Drop guards before unloading
        drop(runtime_a);
        drop(runtime_b);

        // Explicitly unload in reverse order of creation (B then A)
        registry.unload("com.example.b").unwrap();
        registry.unload("com.example.a").unwrap();
    }

    #[test]
    fn test_extension_smoke_storage() {
        let hooks = Arc::new(HookBus::new());
        // Use a real file for persistence test, or :memory: for logic test
        let registry = ExtensionRegistry::new(hooks, std::path::Path::new(":memory:")).unwrap();
        
        let json = r#"{
            "id": "com.example.storage",
            "name": "Storage Test",
            "version": "1.0.0",
            "description": "Storage smoke test",
            "min_folivm_version": "1.0.0",
            "entry": "index.js",
            "permissions": {}
        }"#;

        let agent = registry.load_manifest(json).unwrap();
        let mut runtime_guard = agent.runtime.lock().unwrap();
        let runtime = runtime_guard.as_mut().unwrap();

        // 1. Set a value
        runtime.execute_script("smoke:storage1", "folivm.storage.set('level', 42);").unwrap();
        
        // 2. Get the value in another script execution
        let script = r#"
            const val = folivm.storage.get('level');
            console.log("Storage value retrieved: " + val);
            if (val !== "42") {
                throw new Error("Storage failed! Expected 42, got " + val);
            }
        "#;
        runtime.execute_script("smoke:storage2", script).unwrap();
        
        registry.unload("com.example.storage").unwrap();
    }
}
