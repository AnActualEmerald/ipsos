extern crate clap;
extern crate dirs;
extern crate serde;
extern crate serde_json;

mod application;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Error, Read, Write};
use std::path::PathBuf;

fn main() {
	let matches = application::get_matches();

	if let Some(matches) = matches.subcommand_matches("list") {
		if matches.is_present("shows") {
			list_shows().expect("Couldn't read file");
		} else {
			list_lists().expect("Couldn't read file");
		}
	}

	if let Some(matches) = matches.subcommand_matches("new") {
		let name = matches.value_of("NAME").unwrap_or("generic");
		new_watchlist(name).expect("Something went wrong creating the new watchlist");
		if matches.is_present("switch") {
			load_list(name).unwrap_or_else(|_| panic!("Couldn't switch to watchlist {}", name));
		}
	}

	if let Some(matches) = matches.subcommand_matches("switch") {
		let name = matches.value_of("NAME").unwrap_or("generic");
		load_list(name).unwrap_or_else(|_| panic!("Couldn't switch to watchlist {}", name));
	}

	if let Some(matches) = matches.subcommand_matches("add") {
		add_show(
			matches.value_of("TITLE"),
			matches.value_of("length"),
			matches.value_of("watched"),
			matches.is_present("done"),
		)
		.expect("Couldn't add show");
	}

	if let Some(matches) = matches.subcommand_matches("watch") {
		watch_show(matches.value_of("TITLE").unwrap_or("none")).expect("Couldn't watch the show");
	}

	if let Some(matches) = matches.subcommand_matches("update") {
		update_show(
			matches.value_of("TITLE"),
			matches.value_of("watched"),
			matches.value_of("length"),
			matches.is_present("done"),
		)
		.expect("Couldn't update the show");
	}
}

fn update_show(
	title: Option<&str>,
	watched: Option<&str>,
	len: Option<&str>,
	done: bool,
) -> Result<(), Error> {
	let path = gen_path();
	let mut ml = read_json(&path)?;

	ml.lists.entry(ml.current.clone()).and_modify(|v| {
		let title_p = title.unwrap_or_else(|| v.current.as_str());
		let watch_p = if let Ok(i) = watched.unwrap_or("1").parse::<i32>() {
			i
		} else {
			1
		};
		let len_p = len.and_then(|i| {
			if let Ok(e) = i.parse::<i32>() {
				Some(e)
			} else {
				None
			}
		});
		v.update(String::from(title_p), watch_p, len_p, done);
	});

	save_json(&ml, &path)?;

	println!("Updated show");

	Ok(())
}

fn watch_show(title: &str) -> Result<(), Error> {
	let path = gen_path();
	let mut ml = read_json(&path)?;

	ml.lists.entry(ml.current.clone()).and_modify(|v| {
		v.current = title.to_owned();
	});

	save_json(&ml, &path)?;

	println!("Now watching {}", title);
	Ok(())
}

fn add_show(
	title: Option<&str>,
	len: Option<&str>,
	watched: Option<&str>,
	completed: bool,
) -> Result<(), Error> {
	let path = gen_path();
	let mut ml = read_json(&path)?;
	let mut len_p: i32 = 0;
	let mut watch_p: i32 = 0;
	let title_p = title.unwrap_or("none").to_owned();

	if let Ok(v) = len.unwrap_or("0").parse::<i32>() {
		len_p = v;
	}

	if let Ok(v) = watched.unwrap_or("0").parse::<i32>() {
		watch_p = v;
	}

	let show = Show {
		title: String::from(&title_p),
		length: len_p,
		watched: watch_p,
		completed,
	};

	ml.lists.entry(ml.current.clone()).and_modify(|e| {
		// maybe too much copying but it shouldn't really matter
		e.shows.insert(show.title.clone(), show);
	});

	save_json(&ml, &path)?;

	println!("Added show {} to watchlist {}", title_p, ml.current);
	Ok(())
}

fn load_list(name: &str) -> Result<(), Error> {
	let path = gen_path();
	let mut ml = read_json(&path)?;
	ml.current = name.to_owned();
	save_json(&ml, &path)?;
	println!("Switched to list {}", name);
	Ok(())
}

fn list_shows() -> Result<(), Error> {
	let ml = read_json(&gen_path())?;
	let current = ml.get_current();
	let s = current.get_current();
	println!("Current Shows: ");
	println!(
		"\nCurrently Watching: \n\n   {}:\n       Watched: {}%({}/{})\n       Completed: {}\n",
		s.title,
		((s.watched as f32 / s.length as f32) * 100.0) as i32,
		s.watched,
		s.length,
		s.completed
	);
	current.shows.iter().for_each(|(_, v)| {
		let res = format!(
			"{}:\n    Watched: {}%({}/{})\n    Completed: {}",
			v.title,
			((v.watched as f32 / v.length as f32) * 100.0) as i32,
			v.watched,
			v.length,
			v.completed
		);
		if current.current == v.title {
			// println!("\nCurrently watching:\n{}\n\n", res);
		} else {
			println!("{}\n", res);
		}
	});

	Ok(())
}

fn list_lists() -> Result<(), Error> {
	let ml = read_json(&gen_path())?;
	println!(
		"Current: {}\nShows: {}\n\nLists: {}",
		ml.current,
		ml.get_current().list(),
		ml.list()
	);
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

	println!("Created watchlist {}", name);

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
			if std::fs::read_dir(&path).is_err() {
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
		res.join(", ")
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

impl WatchList {
	fn list(&self) -> String {
		let mut res = vec![];
		for (_, v) in self.shows.iter() {
			res.push(v.title.as_str());
		}
		res.join(", ")
	}

	fn get_shows(&self) -> Vec<&Show> {
		self.shows.values().collect()
	}

	fn get_current(&self) -> &Show {
		&self.shows[&self.current]
	}

	fn update(&mut self, title: String, watch: i32, length: Option<i32>, done: bool) {
		match length {
			Some(v) => {
				self.shows.entry(title).and_modify(|e| {
					e.watched += watch;
					e.completed = done;
					e.length = v;
				});
			}
			None => {
				self.shows.entry(title).and_modify(|e| {
					e.watched += watch;
					e.completed = done;
				});
			}
		}
	}
}

#[derive(Deserialize, Serialize, Debug)]
struct Show {
	title: String,
	length: i32,
	watched: i32,
	completed: bool,
}

#[cfg(test)]
mod tests {

	use super::*;

	#[test]
	fn json_functions() {
		let path = PathBuf::from("./test2.json".to_owned());
		let mut foo = MainList {
			current: "none".to_owned(),
			lists: HashMap::new(),
		};

		read_json(&path).expect("Well this is embarrasing");
		//need to make sure the file exists first, and
		//for whatever reason i've put that functionality in the read function

		foo.lists.insert(
			"test".to_owned(),
			WatchList {
				name: "test".to_owned(),
				current: "none".to_owned(),
				shows: HashMap::new(),
			},
		);
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
		let foo = gen_path();
		let bar = PathBuf::from(format!(
			"{}/.ipsos/lists.json",
			dirs::home_dir().unwrap().display()
		));
		assert_eq!(foo, bar);

		assert_ne!(foo, PathBuf::from("/lists.json".to_owned()));
	}
}
