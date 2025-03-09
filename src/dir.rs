use std::fs::DirBuilder;

#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn dir_path(groud_id: String, artifact_id: String, dir: &str) -> String {
    format!("{}/{}/{}/{}", "src/main/java", groud_id, artifact_id, dir)
}

#[cfg(target_os = "windows")]
pub fn dir_path(groud_id: String, artifact_id: String, dir: &str) -> String {
    format!(
        "{}\\{}\\{}\\{}",
        "src\\main\\java", groud_id, artifact_id, dir
    )
}

pub fn create_dir(groud_id: String, artifact_id: String) {
    //dir_path(groud_id, artifact_id)
    DirBuilder::new()
        .recursive(true)
        .create(dir_path(
            groud_id.clone(),
            artifact_id.clone(),
            "controller",
        ))
        .unwrap();
    DirBuilder::new()
        .recursive(true)
        .create(dir_path(groud_id.clone(), artifact_id.clone(), "model"))
        .unwrap();
    DirBuilder::new()
        .recursive(true)
        .create(dir_path(
            groud_id.clone(),
            artifact_id.clone(),
            "repository",
        ))
        .unwrap();
    DirBuilder::new()
        .recursive(true)
        .create(dir_path(groud_id.clone(), artifact_id.clone(), "service"))
        .unwrap();
}
