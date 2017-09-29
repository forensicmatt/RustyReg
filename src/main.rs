#[macro_use] extern crate log;
extern crate serde_json;
extern crate env_logger;
extern crate jmespath;
extern crate clap;
extern crate rwinreg;
use rwinreg::hive;
use clap::{App, Arg, ArgMatches};
use jmespath::{Expression};
use std::fs;

fn process_directory(directory: &str, options: &ArgMatches) {
    for dir_reader in fs::read_dir(directory) {
        for entry_result in dir_reader {
            match entry_result {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() {
                        let path_string = path.into_os_string().into_string().unwrap();
                        match hive::has_hive_signature(&path_string){
                            Ok(result) => {
                                match result {
                                    true => {
                                        process_file(&path_string, &options);
                                    },
                                    false => {
                                        debug!("{} is not a hive file.",path_string);
                                    }
                                }
                            },
                            Err(error) => {
                                error!("Error testing signature for {} [{:?}]",path_string,error);
                            }
                        }
                    } else if path.is_dir(){
                        let path_string = path.into_os_string().into_string().unwrap();
                        process_directory(&path_string, &options);
                    }
                },
                Err(error) => {
                    error!("Error reading {} [{:?}]",directory,error);
                }
            }
        }
    }
}

fn process_file(filename: &str, options: &ArgMatches) -> bool {
    info!("processing file: {}",filename);

    // JMES Expression if needed
    let mut expr: Option<Expression> = None;
    if options.is_present("query") {
        expr = Some(jmespath::compile(
            options.value_of("query").unwrap()
        ).unwrap());
    }

    // Expression bool flag
    let mut expr_as_bool = false;
    if options.is_present("bool_expr"){
        expr_as_bool = true;
    }

    let hive = match hive::Hive::new(filename) {
        Ok(hive) => hive,
        Err(error) => {
            error!("{} [error: {}]", filename, error);
            return false;
        }
    };

    for value in hive {
        let json_str = serde_json::to_string(&value).unwrap();
        match expr {
            Some(ref j_expr) => {
                let data = jmespath::Variable::from_json(&json_str).unwrap();
                let result = j_expr.search(data).unwrap();
                if expr_as_bool {
                    match result.as_boolean() {
                        Some(bool_value) => {
                            match bool_value {
                                true => println!("{}",json_str),
                                false => {}
                            }
                        },
                        None => {
                            error!("Query expression is not a bool expression! [{}]",options.value_of("query").unwrap());
                        }
                    }
                } else {
                    println!("{}",result)
                }
            },
            None => {
                println!("{}",json_str);
            }
        }
    }

    return true;
}

fn is_directory(source: &str)->bool{
    let metadata = fs::metadata(source).unwrap();

    let file_type = metadata.file_type();

    file_type.is_dir()
}

fn main() {
    env_logger::init().unwrap();

    let source_arg = Arg::with_name("source")
        .short("s")
        .long("source")
        .value_name("PATH")
        .help("Source.")
        .required_unless("pipe")
        .takes_value(true);

    let jmes_arg = Arg::with_name("query")
        .short("q")
        .long("query")
        .value_name("QUERY")
        .help("JMES Query")
        .takes_value(true);

    let bool_arg = Arg::with_name("bool_expr")
        .short("b")
        .long("bool_expr")
        .help("JMES Query as bool only. (Prints whole record if true.)");

    let options = App::new("RusyReg")
        .version("0.1.1")
        .author("Matthew Seyer <https://github.com/forensicmatt/RustyReg>")
        .about("Registry Parser written in Rust.")
        .arg(source_arg)
        .arg(jmes_arg)
        .arg(bool_arg)
        .get_matches();

    let source = options.value_of("source").unwrap();

    if is_directory(source) {
        process_directory(source,&options);
    } else {
        process_file(source,&options);
    }
}
