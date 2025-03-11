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
                let type_tab: Vec<String>;
                type_tab = sqlf::read_from_sql(String::from("init.sql"), argv[i + 1].clone());
                println!("{}", type_tab[0]);
                i += 1;
            }
            _ => {
                println!("{}", argv[i]);
                let (group_id, artifact_id) = read_saka();
                javafile::create_files(&group_id, &artifact_id, argv[i].clone());
            }
        }

        i += 1;
    }
}
