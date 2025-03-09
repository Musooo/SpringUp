use crate::dir::dir_path;
use std::fs;

pub fn get_group_id_slash(group_id: String) -> String {
    let group_ids: String;
    if cfg!(target_os = "linux") || cfg!(target_os = "macos") {
        group_ids = group_id.clone().replace(".", "/");
    } else {
        group_ids = group_id.clone().replace(".", "\\");
    }

    return group_ids;
}

pub fn create_files(group_id: String, artifact_id: String, name: String) {
    let group_ids: String = get_group_id_slash(group_id);
    fs::write(
        format!(
            "{}{}.java",
            dir_path(group_ids.clone(), artifact_id.clone(), "model"),
            name
        ),
        "",
    )
    .expect("fallito creare il file model");
    fs::write(
        format!(
            "{}{}Service.java",
            dir_path(group_ids.clone(), artifact_id.clone(), "service"),
            name
        ),
        "",
    )
    .expect("fallito creare il file model");
    fs::write(
        format!(
            "{}{}Repository.java",
            dir_path(group_ids.clone(), artifact_id.clone(), "repository"),
            name
        ),
        "",
    )
    .expect("fallito creare il file model");
    return;
}
