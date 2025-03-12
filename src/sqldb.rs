use mysql::prelude::*;
use mysql::*;
use crate::sqlf;

pub fn connect_db(table_name: String) -> Vec<String> {
    let url = "mysql://root:root@localhost:3306/noleggiofilm";
    let pool = Pool::new(url).expect("error");

    let mut conn = pool.get_conn().expect("error");
    let result: Vec<(String, String)> = conn.query(format!("SHOW CREATE TABLE {}", table_name)).expect("error");
    let mut i:i32 = 0;
    let mut sql_fields: Vec<String> = Vec::new();

    for row in result {
        for db_field in row.1.split("\n"){
            if i != 0 {
                sql_fields.push(String::from(db_field.trim_start()));
            }
            i +=1;
        }
    }

    sql_fields.pop();
    
    return sql_fields
        .iter()
        .map(|x| sqlf::translate_sql_to_java_type(x.to_string()))
        .collect();
}
