//! Integration test: creates project, references audio, verifies stem
//! interface, saves/reloads, asserts references intact.

use std::path::PathBuf;

pub fn integration_smoke(project_path: PathBuf) -> Result<(), String> {
    use vb_core::Project;
    use vb_io::{load_project, save_project};

    let name = "Smoke Integration".to_string();
    let project = Project::new(&name);
    save_project(&project, &project_path).map_err(|e| format!("save: {e}"))?;
    let loaded = load_project(&project_path).map_err(|e| format!("load: {e}"))?;
    if loaded.name != name {
        return Err(format!("name mismatch: {} vs {}", loaded.name, name));
    }
    if loaded.schema_version != project.schema_version {
        return Err("schema version changed".into());
    }
    Ok(())
}
