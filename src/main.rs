use std::env;
mod pom;

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
                pom::read_from_pom();
            }
            "-d" => {
                println!("create dir");
            }
            "-s" => {
                if argv.len() < i + 2 {
                    println!("not enough args for -s");
                    return;
                }
                pom::set_ids(argv[i + 1].clone(), argv[i + 2].clone());
                i += 2;
            }
            _ => {
                println!("{}", argv[i]);
            }
        }

        i += 1;
    }
}
