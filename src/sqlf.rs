use std::fs;

pub fn read_from_sql(file_name: String, table_name: String) -> Vec<String> {
    let type_tab: Vec<String> = fs::read_to_string(file_name)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut sql_fields: Vec<String> = Vec::new();

    let mut i: usize = 0;
    let mut cond: bool = false;

    while type_tab.len() != i {
        if type_tab[i].contains(&"CREATE TABLE") && type_tab[i].contains(&table_name.to_lowercase())
        {
            cond = true;
        } else if cond {
            if type_tab[i].contains(&";") {
                break;
            }
            sql_fields.push(String::from(type_tab[i].trim_start()));
        }
        i += 1;
    }

    return sql_fields
        .iter()
        .map(|x| translate_sql_to_java_type(x.to_string()))
        .collect();
}

pub fn translate_sql_to_java_type(row: String) -> String {
    let row_vec: Vec<&str> = row.split(" ").collect();
    /*
    for l in &row_vec {
        println!("{}", l);
        println!("{}", l.contains("varchar"));
    }*/

    let java_type = match String::from(row_vec[1]) {
        x if x.contains("varchar") => "String",
        x if x.contains("int") => "Integer",
        x if x.contains("bool") => "Boolean",
        x if x.contains("KEY") => "A",
        _ => {
            println!("{}", row_vec[1]);
            return "Erorr".to_string();
        }
    };

    if java_type == "A" {
        return String::from(
            row_vec[2]
                .replace("`", "")
                .replace("(", "")
                .replace(")", ""),
        );
    }

    return format!(
        "   private {} {};\n",
        java_type,
        String::from(row_vec[0].replace("`", ""))
    );
}
