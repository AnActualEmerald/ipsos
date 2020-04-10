extern crate clap;
extern crate dirs;
extern crate serde;
extern crate serde_json;

use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
// use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Error, Read, Write};
use std::path::PathBuf;

fn main() {
    let matches = App::new("Ipsos Watchlist Manager")
        .version("0.0.1")
        .author("Emerald")
        .about("Manage your watchlists with ease from the command line")
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("Show all the current watchlists"),
        )
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("Create a new watchlist")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the new watchlist")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("switch")
                .alias("s")
                .about("Switch to an existing watchlist")
                .arg(
                    Arg::with_name("NAME")
                        .help("Name of the watchlist to switch to")
                        .required(true),
                ),
        )
        .get_matches();

    if matches.is_present("list") {
        list_lists().expect("Couldn't read file");
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        new_watchlist(matches.value_of("NAME").unwrap_or("generic"))
            .expect("Something went wrong creating the new watchlist");
    }

    if let Some(matches) = matches.subcommand_matches("switch") {
        let name = matches.value_of("NAME").unwrap_or("generic");
        load_list(name).expect(format!("Couldn't switch to watchlist {}", name).as_str());
    }
}

fn load_list(name: &str) -> Result<(), Error> {
    let path = gen_path();
    let mut ml = read_json(&path)?;
    ml.current = name.to_owned();
    save_json(&ml, &path)?;
    Ok(())
}

fn list_lists() -> Result<(), Error> {
    let ml = read_json(&gen_path())?;
    println!("Current: {}\n\nLists: {}", ml.current, ml.list());
    Ok(())
}

fn new_watchlist(name: &str) -> Result<(), Error> {
    let path = gen_path();
    let mut list = read_json(&path)?;

    //add the provided watchlist name to the list of lists
    list.lists.insert(
        String::from(name),
        WatchList {
            name: String::from(name),
            current: "none".to_owned(),
            shows: HashMap::new(),
        },
    );

    save_json(&list, &path)?;

    Ok(())
}

//utility functions
fn save_json(data: &MainList, path: &PathBuf) -> Result<(), Error> {
    let mut op = OpenOptions::new();
    let mut file = match op.write(true).truncate(true).open(&path) {
        Err(e) => panic!("Couldn't open file {}: {:?}", path.display(), e),
        Ok(file) => file,
    };

    file.write_all(serde_json::to_string(&data).unwrap().as_bytes())
}

fn read_json(path: &PathBuf) -> Result<MainList, Error> {
    let data: MainList;
    let mut op = OpenOptions::new();

    let mut file = match op.read(true).write(true).create(true).open(&path) {
        Err(e) => {
            if let Err(_) = std::fs::read_dir(&path) {
                std::fs::create_dir(PathBuf::from(format!(
                    "{}/.ipsos",
                    dirs::home_dir().unwrap().display()
                )))?; //if the directory doesn't exist, create it and try again
                return read_json(&path);
            } else {
                panic!("Couldn't open file {}: {:?}", path.display(), e) //if that isn't the issue, something else is wrong
            }
        }
        Ok(file) => file,
    };

    let mut raw: String = String::new();
    if let Err(e) = file.read_to_string(&mut raw) {
        panic!("Got this error when reading the file: {}", e);
    };
    if raw == "{}/n" || raw == "" {
        data = MainList {
            current: "none".to_owned(),
            lists: HashMap::new(),
        };
    } else {
        let res: Result<MainList, serde_json::Error> = serde_json::from_str(raw.as_str());

        data = match res {
            Err(e) => panic!("Got this error when trying to deserialize json: {}", e),
            Ok(j) => j,
        };
    }

    Ok(data)
}

fn gen_path() -> PathBuf {
    let r_path = format!("{}/.ipsos/lists.json", dirs::home_dir().unwrap().display()); //I want this to be in the user's home directory
    PathBuf::from(r_path)
}

//structs
#[derive(Deserialize, Serialize, Debug)]
struct MainList {
    current: String,
    lists: HashMap<String, WatchList>,
}

impl MainList {
    fn list(&self) -> String {
        let mut res = vec![];
        for (_, v) in self.lists.iter() {
            res.push(v.name.as_str());
        }
        format!("{}", res.join(", "))
    }

    fn get_current(&self) -> &WatchList {
        &self.lists[&self.current]
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct WatchList {
    name: String,
    current: String,
    shows: HashMap<String, Show>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Show {
    title: String,
    length: i32,
    watched: i32,
    completed: bool,
}
