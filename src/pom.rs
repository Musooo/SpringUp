use std::fs;

pub fn set_ids(group_id: String, artifact_id: String, conf_file: &str) -> std::io::Result<()> {
    fs::write(conf_file, format!("{}\n{}\n", group_id, artifact_id))?;
    Ok(())
}

pub fn read_from_pom(conf_file: &str) -> std::io::Result<()> {
    let pom_content: Vec<String> = fs::read_to_string("pom.xml")
        .unwrap()
        .lines()
        .map(String::from)
        .filter(|l| l.contains("groupId") || l.contains("artifactId"))
        .collect();

    fs::write(
        conf_file,
        format!(
            "{}\n{}\n",
            pom_content[2]
                .trim()
                .replace("<groupId>", "")
                .replace("</groupId>", ""),
            pom_content[3]
                .trim()
                .replace("<artifactId>", "")
                .replace("</artifactId>", "")
        ),
    )?;

    Ok(())
}
