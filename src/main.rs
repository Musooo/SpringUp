use std::env;
mod pom;
use std::fs;
mod dir;
mod javafile;
mod sqlf;

macro_rules! conf_file {
    () => {
        "prova.saka"
    };
}

fn read_saka() -> (String, String) {
    let saka_content: Vec<String> = fs::read_to_string(conf_file!())
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    (saka_content[0].clone(), saka_content[1].clone())
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let mut i: usize = 1;

    if argv.len() == 1 {
        println!("use -h for help");
        return;
    }

    while i < argv.len() {
        match argv[i].as_str() {
            "-h" => {
                println!("help");
            }
            "-a" => {
                println!("pom parser");
                let _ = pom::read_from_pom(conf_file!());
            }
            "-d" => {
                println!("create dir");
                let (group_id, artifact_id) = read_saka();
                let group_ids: String = javafile::get_group_id_slash(group_id);
                dir::create_dir(&group_ids, &artifact_id);
            }
            "-s" => {
                if argv.len() < i + 2 {
                    println!("not enough args for -s");
                    return;
                }
                let _ = pom::set_ids(&argv[i + 1], &argv[i + 2], conf_file!());
                i += 2;
            }
            "-f" => {
                let mut tabel_name_different: bool = false;
                let mut tabel_name_different_and_name_given: bool = false;

                let mut type_tab: Vec<String>;
                if argv.len() <= i + 1 {
                    println!("missing the name");
                    return;
                } else if argv.len() <= i + 2 {
                    type_tab = sqlf::read_from_sql(String::from("init.sql"), argv[i + 1].clone());
                } else {
                    if argv[i + 2] == "-t" {
                        type_tab = sqlf::read_from_sql(argv[i + 3].clone(), argv[i + 1].clone());
                        tabel_name_different_and_name_given = true;
                    } else {
                        println!("no .sql file name gave");
                        type_tab =
                            sqlf::read_from_sql(String::from("init.sql"), argv[i + 1].clone());
                    }
                    tabel_name_different = true;
                }
                //type_tab = sqlf::read_from_sql(String::from("init.sql"), argv[i + 1].clone());
                let mut id_type = String::from("Tipo");

                for j in 0..type_tab.len() {
                    if type_tab[j].contains(&type_tab[type_tab.len() - 1]) {
                        if j == 0 {
                            type_tab.insert(0, String::from("   @Id\n"));
                        } else {
                            type_tab.insert(j - 1, String::from("   @Id\n"));
                        }
                        let id_vec: Vec<String> = type_tab[j + 1]
                            .clone()
                            .trim_start()
                            .split(" ")
                            .map(String::from)
                            .collect();
                        id_type = id_vec[1].clone();
                        break;
                    }
                }

                type_tab.pop();

                let text: String = type_tab.into_iter().collect();
                let (group_id, artifact_id) = read_saka();
                javafile::create_files(
                    &group_id,
                    &artifact_id,
                    argv[i + 1].clone(),
                    text,
                    &id_type,
                );
                if tabel_name_different {
                    i += 1
                };
                if tabel_name_different_and_name_given {
                    i += 1
                };
                i += 1;
            }
            _ => {
                println!("{}", argv[i]);
                let (group_id, artifact_id) = read_saka();
                javafile::create_files(
                    &group_id,
                    &artifact_id,
                    argv[i].clone(),
                    String::from(""),
                    &String::from("Tipo"),
                );
            }
        }

        i += 1;
    }
}
