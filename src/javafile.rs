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

pub fn create_files(
    group_id: &String,
    artifact_id: &String,
    name: String,
    model_text: String,
    id_type: &str,
) {
    let group_ids: String = get_group_id_slash(group_id.clone());
    fs::write(
        format!(
            "{}{}.java",
            dir_path(&group_ids, artifact_id, "model"),
            name
        ),
        format!("package {}.{}.model;\n\nimport jakarta.persistence.Entity;\nimport jakarta.persistence.Id;\nimport jakarta.persistence.Table;\nimport lombok.AllArgsConstructor;\nimport lombok.Getter;\nimport lombok.NoArgsConstructor;\nimport lombok.Setter;\n\n@Entity\n@Getter\n@Setter\n@AllArgsConstructor\n@NoArgsConstructor\n@Table(name = \"{}\")\n\npublic class {} {{\n{}}}", group_id, artifact_id, name.to_lowercase(), name, model_text),)
    .expect("fallito creare il file model");
    fs::write(
        format!(
            "{}{}Service.java",
            dir_path(&group_ids, artifact_id, "service"),
            name
        ),
        format!("package {}.{}.service;\n\nimport org.springframework.beans.factory.annotation.Autowired;\nimport org.springframework.stereotype.Service;\nimport {}.{}.model.{};\nimport {}.{}.repository.{}Repository;\n\nimport java.util.List;\n\n@Service\npublic class {}Service {{\n   @Autowired\n   private {}Repository {}Repository;\n\n   public List<{}> findAll() {{\n      return {}Repository.findAll();\n   }}\n\n   public {} findById({} id) {{\n      return {}Repository.findById(id).orElse(null);\n   }}\n\n   public {} save({} {}) {{\n      return {}Repository.save({});\n   }}\n\n}}", group_id, artifact_id, group_id, artifact_id, name, group_id, artifact_id, name, name, name, name.to_lowercase(), name, name.to_lowercase(), name, id_type , name.to_lowercase(), name, name, name.to_lowercase(), name.to_lowercase(), name.to_lowercase()),
    )
    .expect("fallito creare il file model");
    fs::write(
        format!(
            "{}{}Repository.java",
            dir_path(&group_ids, artifact_id, "repository"),
            name
        ),
        format!("package {}.{}.repository;\n\nimport org.springframework.data.jpa.repository.JpaRepository;\nimport org.springframework.stereotype.Repository;\nimport {}.{}.model.{};\n@Repository\npublic interface {}Repository extends JpaRepository<{}, {}> {{}}", group_id, artifact_id, group_id, artifact_id, name, name, name, id_type),
    )
    .expect("fallito creare il file model");
    return;
}
