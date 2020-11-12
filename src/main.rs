extern crate clap;
extern crate dirs;
extern crate serde;
extern crate serde_json;

mod application;
mod manager;
mod imdb;

#[macro_use] extern crate prettytable;

#[tokio::main]
async fn main() {
	let matches = application::get_matches();

	if let Some(matches) = matches.subcommand_matches("list") {
		if matches.is_present("lists") {
			manager::list_lists().expect("Couldn't read file");
		} else {
			manager::list_shows().expect("Couldn't read file");
		}
	}

	if let Some(matches) = matches.subcommand_matches("new") {
		let name = matches.value_of("NAME").unwrap_or("generic");
		manager::new_watchlist(name).expect("Something went wrong creating the new watchlist");
		if matches.is_present("switch") {
			manager::load_list(name)
				.unwrap_or_else(|_| panic!("Couldn't switch to watchlist {}", name));
		}
	}

	if let Some(matches) = matches.subcommand_matches("switch") {
		let name = matches.value_of("NAME").unwrap_or("general");
		manager::load_list(name)
			.unwrap_or_else(|_| panic!("Couldn't switch to watchlist {}", name));
	}

	if let Some(matches) = matches.subcommand_matches("add") {
		if matches.is_present("imdb"){
			if let Err(_e) = manager::add_show_imdb(&matches.values_of("TITLE").unwrap().collect::<String>()).await {
				println!("Coudln't add that show");
			}
		}else{
			manager::add_show(
				Some(&matches.values_of("TITLE").unwrap().collect::<String>()),
				matches.value_of("length"),
				matches.value_of("watched"),
				matches.is_present("done"),
			)
			.expect("Couldn't add show");
		}
	}

	if let Some(matches) = matches.subcommand_matches("watch") {
		manager::watch_show(matches.value_of("TITLE").unwrap_or("none"))
			.expect("Couldn't watch the show");
	}

	if let Some(matches) = matches.subcommand_matches("update") {
		manager::update_show(
			matches.value_of("TITLE"),
			matches.value_of("length"),
			matches.is_present("done"),
		)
		.expect("Couldn't update the show");
	}

	if let Some(matches) = matches.subcommand_matches("remove") {
		let id = matches.value_of("ID");
		if let Some(tmp) = id {
			match manager::remove_show_id(tmp) {
				Ok(s) => println!("Removed show {}", s),
				Err(e) => println!("Coudln't remove show at id {}: {}", id.unwrap(), e)
			}
		}else {
			if let Some(title) = matches.value_of("title") {
				manager::remove_show(title);
				println!("Removed show {}", title);
			}else {
				println!("Need a title or ID");
			}
		}
	}
}

//testing
#[cfg(test)]
mod tests {

	use super::*;
	use manager::*;
	use std::collections::HashMap;
	use std::path::PathBuf;

	#[test]
	fn json_functions() {
		let path = PathBuf::from("./test2.json".to_owned());


		read_json(&path).expect("Well this is embarrasing");
		//need to make sure the file exists first, and
		//for whatever reason i've put that functionality in the read function


		let foo = WatchList {
				name: "test".to_owned(),
				current: "none".to_owned(),
				shows: HashMap::new(),
			};

		save_json(&foo, &path).expect("Well this is embarrasing");

		if let Ok(bar) = read_json(&path) {
			assert_eq!(foo.current, bar.current);
			assert_eq!(foo.list(), bar.list());
		}
	}
	#[test]
	fn json_init() {
		if let Ok(foo) = read_json(&PathBuf::from("./test.json".to_owned())) {
			let bar = MainList {
				current: "none".to_owned(),
				lists: HashMap::new(),
			};
			assert_eq!(foo.current, bar.current);
		}
	}

	#[test]
	fn path_generation() {
		//this might fail on my local machine beacues the config file will likely be edited
		let foo = gen_path();
		let bar = PathBuf::from(format!(
			"{}/.ipsos/general.json",
			dirs::home_dir().unwrap().display()
		));
		assert_eq!(foo, bar);

		assert_ne!(foo, PathBuf::from("/lists.json".to_owned()));
	}
}
