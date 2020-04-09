extern crate clap;
extern crate dirs;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use serde_json::{json, Value};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;

// const FILEPATH: &'static str = "lists.json";

fn main() {
    let matches = App::new("Ipsos Watchlist Manager")
        .version("0.0.1")
        .author("Emerald")
        .about("Manage your watchlist with ease from the command line")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Show all the current watchlists"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .about("Create a new watchlist")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the new watchlist")
                        .required(true),
                ),
        )
        .get_matches();
    if matches.is_present("list") {
        println!("A whole bunch of watchlists");
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        new_watchlist(matches.value_of("NAME").unwrap_or("generic"));
    }
}

// #[derive(Deserialize, Debug)]
// enum Objects {
//     WatchList {
//         name: String,
//         contents: Vec<Objects>,
//     },
//     Show {
//         name: String,
//         length: i32,
//         watched: i32,
//         completed: bool,
//     },
// }

fn new_watchlist(name: &str) {
    let r_path = format!("{}/.ipsos/lists.json", dirs::home_dir().unwrap().display()); //I want this to be in the user's home directory
    let path = Path::new(&r_path);
    let mut list: Value;
    let mut op = OpenOptions::new();
    {
        let mut file = match op.read(true).write(true).open(&path) {
            Err(e) => panic!("Couldn't open file {}: {:?}", path.display(), e),
            Ok(file) => file,
        };

        let mut raw: String = String::new();
        if let Err(e) = file.read_to_string(&mut raw) {
            panic!("Got this error when reading the file: {}", e);
        };
        if raw == "{}/n" || raw == "" {
            list = json!({
                "current": null,
                "lists": {}
            })
        } else {
            let res: Result<Value, serde_json::Error> = serde_json::from_str(raw.as_str());

            list = match res {
                Err(e) => panic!("Got this error when trying to deserialize json: {}", e),
                Ok(j) => j,
            };
        }
    }
    list["lists"][name] = json!(vec!(""));

    {
        let mut file = match op.write(true).truncate(true).open(&path) {
            Err(e) => panic!("Couldn't open file {}: {:?}", path.display(), e),
            Ok(file) => file,
        };

        if let Err(e) = file.write_all(serde_json::to_string(&list).unwrap().as_bytes()) {
            println!("There was an issue writing the file: {}", e);
        };
    }
}
