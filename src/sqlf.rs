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
        if type_tab[i].contains(&"CREATE") && type_tab[i].contains(&table_name) {
            cond = true;
        } else if cond {
            if type_tab[i].contains(&";") {
                break;
            }
            sql_fields.push(type_tab[i].clone());
        }
        i += 1;
    }

    return sql_fields;
}
