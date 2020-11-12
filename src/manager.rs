use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Error, Read, Write};
use std::path::PathBuf;
use prettytable::{Table, format};
use toml;

use crate::imdb;

pub fn update_show(
	title: Option<&str>,
	len: Option<&str>,
	done: bool,
) -> Result<(), String> {
	let path = gen_path();
	let mut wl = read_json(&path)?;

	let title_p = title.unwrap_or_else(|| wl.current.as_str());
	let len_p = if let Some(tmp) = len {
			Some(tmp.to_owned())
		} else {
			None
		};
	wl.update(String::from(title_p), len_p, done);


	save_json(&wl, &path)?;

	println!("Updated show");

	Ok(())
}

pub fn watch_show(title: &str) -> Result<(), String> {
	let path = gen_path();
	let mut wl = read_json(&path)?;

	
	wl.current = title.to_owned();
	

	save_json(&wl, &path)?;

	println!("Now watching {}", title);
	Ok(())
}

pub async fn add_show_imdb(title: &str) -> Result<(), String>{
	match imdb::get_show_data(title).await {
		Some(mut show) => {
			let path = gen_path();
			let mut wl = read_json(&path)?;
			let mut cfg = read_config();
			cfg.id += 1;
			save_config(&cfg);
	
			show.id = cfg.id;
			wl.shows.insert(show.title.clone(), show.clone());

			save_json(&wl, &path)?;

			println!("Added show {} to watchlist {}", show.title, wl.name);
			Ok(())
		}
		_ => Ok(())
	}
}

pub fn add_show(
	title: Option<&str>,
	len: Option<&str>,
	watched: Option<&str>,
	completed: bool,
) -> Result<(), String> {
	let path = gen_path();
	let mut wl = read_json(&path)?;
	let mut cfg = read_config();
	let mut len_p = String::new();
	let title_p = title.unwrap_or("none").to_owned();

	cfg.id += 1;
	save_config(&cfg);
	
	if let Some(v) = len{
		len_p = v.to_owned();
	}
	
	let show = Show {
		id: cfg.id,
		title: String::from(&title_p),
		runtime: len_p,
		completed,
	};

	
		// maybe too much copying but it shouldn't really matter
	wl.shows.insert(show.title.clone(), show);
	

	save_json(&wl, &path)?;

	println!("Added show {} to watchlist {}", title_p, wl.name);
	Ok(())
}

pub fn remove_show(title: &str) -> Result<(), String> {
	let path = gen_path();
	let mut wl = read_json(&path)?;
	wl.shows.remove(title);

	save_json(&wl, &path);
	Ok(())
}

pub fn remove_show_id(id: &str) -> Result<String, String>{
	//need to implement ID's first
	let path = gen_path();
	let mut wl = read_json(&path)?;
	let mut show = None;
	for (k, v) in wl.shows.iter() {
		if Ok(v.id) == id.parse::<i32>() {
			show = Some(k.clone());
			break;
		}else {
			continue;
		}
	}

	if let Some(s) = show {
		wl.shows.remove(&s);
		save_json(&wl, &path);
		Ok(s)
	}else {
		Err(String::from("Couldn't find show to delete"))
	}
}

pub fn load_list(name: &str) -> Result<(), String> {
	let mut cfg = read_config();
	cfg.lists.push(name.to_string()); // add the name to the list of lists in the config file
	println!("Switched to list {}", name);
	Ok(())
}

pub fn list_shows() -> Result<(), String> {
	let current = read_json(&gen_path())?;
	let mut table = Table::new();
	let format = format::FormatBuilder::new()
		.column_separator('|')
		.borders('|')
		.padding(1, 1)
		.separators(&[format::LinePosition::Top, format::LinePosition::Intern,format::LinePosition::Bottom], format::LineSeparator::new('-', '+', '+', '+'))
		.separator(format::LinePosition::Title, format::LineSeparator::new('=', '+', '+', '+'))
		.build();
	table.set_format(format);
	table.set_titles(row!["", "ID", "Title", "Runtime", "Done"]);

	let mut sort: Vec<(&String, &Show)>= current.shows.iter().collect();

	sort.sort_by(|a, b| {
		a.1.id.cmp(&b.1.id)
	});

	sort.iter().for_each(|(_, v)| {
		if current.current == v.title {
			table.add_row(row![">", v.id, v.title, v.runtime, v.completed]);

		} else {
			table.add_row(row!["", v.id, v.title, v.runtime, v.completed]);
		}
	});

	if current.shows.len() == 0 {
		table.add_row(row!["", "", "", "", ""]);
	}

	table.printstd();

	Ok(())
}

pub fn list_lists() -> Result<(), String> {
	let wl = read_json(&gen_path())?;
	let cfg = read_config();
	// let lists = match cfg.lists {
	// 	Some(l) => l,
	// 	None => vec![]
	// };
	println!(
		"Current: {}\nShows: {}\n\nLists: {}",
		cfg.current_list.unwrap(),
		wl.list(),
		cfg.lists.join(", ")
	);
	Ok(())
}

pub fn new_watchlist(name: &str) -> Result<(), String> {
	let mut cfg = read_config();
	
	

	//add the provided watchlist name to the list of lists
	cfg.lists.push(name.to_string());
	save_config(&cfg);

	println!("Created watchlist {}", name);

	Ok(())
}

//utility functions
pub fn save_json(data: &WatchList, path: &PathBuf) -> Result<(), String> {
	let mut op = OpenOptions::new();
	let mut file = match op.write(true).truncate(true).open(&path) {
		Err(e) => return Err(format!("Couldn't open file {}: {:?}", path.display(), e)),
		Ok(file) => file,
	};

	match file.write_all(serde_json::to_string(&data).unwrap().as_bytes()) {
		Ok(_) => Ok(()),
		Err(e) => return Err(format!("Couldn't write file {}: {:?}", path.display(), e))
	}
}

pub fn read_json(path: &PathBuf) -> Result<WatchList, String> {
	let data: WatchList;
	let mut op = OpenOptions::new();

	let mut file = match op.read(true).write(true).create(true).open(&path) {
		Err(e) => {
			if std::fs::read_dir(&path).is_err() {
				std::fs::create_dir(PathBuf::from(format!(
					"{}/.ipsos",
					dirs::home_dir().unwrap().display()
				))).unwrap(); //if the directory doesn't exist, create it and try again
				return read_json(&path);
			} else {
				panic!("Couldn't open file {}: {:?}", path.display(), e) //if that isn't the issue, something else is wrong
			}
		}
		Ok(file) => file,
	};

	let mut raw: String = String::new();
	if let Err(e) = file.read_to_string(&mut raw) {
		return Err(format!("Got this error when reading the file: {}", e));
	}
	if raw == "{}/n" || raw == "" {
		data = WatchList {
			name: "general".to_owned(),
			current: "none".to_owned(),
			shows: HashMap::new(),
		};
	} else {
		let res: Result<WatchList, serde_json::Error> = serde_json::from_str(raw.as_str());

		data = match res {
			Err(e) => panic!("Got this error when trying to deserialize json: {}", e),
			Ok(j) => j,
		};
	}

	Ok(data)
}

pub fn save_config(cfg: &Config) -> Result<(), String>{
	let mut op = OpenOptions::new();
	if let Some(mut path) = dirs::config_dir(){
		path.push("ipsos/config.toml");
		let mut file = match op.write(true).truncate(true).open(&path) {
			Err(e) => return Err(format!("Couldn't open file {}: {:?}", path.display(), e)),
			Ok(file) => file,
		};

		match file.write_all(toml::to_string(&cfg).unwrap().as_bytes()) {
			Ok(_) => return Ok(()),
			Err(e) => return Err(format!("Couldn't write file {}: {:?}", path.display(), e))
		}
	}

	Ok(())
}

pub fn read_config() -> Config {
	if let Some(mut dir) = dirs::config_dir(){
		dir.push("ipsos");
		if std::fs::read_dir(&dir).is_err() {
			std::fs::create_dir(PathBuf::from(format!(
				"{}/ipsos",
				dirs::config_dir().unwrap().display()
			))).unwrap(); //if the directory doesn't exist, create it and try again
			save_config(&Config{
				current_list: Some("general".to_string()),
				lists: vec![],
				id: 0,
			}).expect("Unable to write config file");
			return read_config();
		}else {
			dir.push("config.toml");
		}
		let mut op = OpenOptions::new();
		let mut file = match op.read(true).write(true).create(true).open(&dir) {
			Err(e) => {
					panic!("Couldn't open file {}: {:?}", dir.display(), e) //if that isn't the issue, something else is wrong
			}
			Ok(file) => file,
		};

		let mut buf = String::new();
		file.read_to_string(&mut buf);
		if buf == String::new() {
			buf = String::from(r#"
				current_list = "general"
				lists = []
				id = 0
			"#);
		}


		let config: Config = toml::from_str(&buf).unwrap_or_else(|e| panic!("{}", e));
		return config;
	}else {
		panic!("Can't access config directory");
	}
}

pub fn gen_path() -> PathBuf {
	let mut config = read_config();
	if config.current_list == None{
		config.current_list = Some("general".to_string());
		save_config(&config);
	}
	let r_path = format!("{}/.ipsos/{}.json", dirs::home_dir().unwrap().display(), config.current_list.unwrap());
	PathBuf::from(r_path)
}

//structs
#[derive(Deserialize, Serialize, Debug)]
pub struct MainList {
	pub current: String,
	pub lists: HashMap<String, WatchList>,
}

impl MainList {
	pub fn list(&self) -> String {
		let mut res = vec![];
		for (_, v) in self.lists.iter() {
			res.push(v.name.as_str());
		}
		res.join(", ")
	}

	fn get_current(&self) -> Result<&WatchList, String> {
		if self.lists.contains_key(&self.current) {
			Ok(&self.lists[&self.current])
		}else {
			Err(format!("Couldn't find list {}", &self.current))
		}
	}
}

#[derive(Deserialize, Serialize, Debug)]
pub struct WatchList {
	pub name: String,
	pub current: String,
	pub shows: HashMap<String, Show>,
}

impl WatchList {
	pub fn list(&self) -> String {
		let mut res = vec![];
		for (_, v) in self.shows.iter() {
			res.push(v.title.as_str());
		}
		res.join(", ")
	}

	fn get_shows(&self) -> Vec<&Show> {
		self.shows.values().collect()
	}

	fn get_current(&self) -> Option<&Show> {
		if self.shows.contains_key(&self.current) {
			Some(&self.shows[&self.current])
		}else {
			None
		}
	}

	fn update(&mut self, title: String, length: Option<String>, done: bool) {
		match length {
			Some(v) => {
				self.shows.entry(title).and_modify(|e| {
					e.completed = done;
					e.runtime = v;
				});
			}
			None => {
				self.shows.entry(title).and_modify(|e| {
					e.completed = done;
				});
			}
		}
	}
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Show {
	pub id: i32,
	pub title: String,
	pub runtime: String,
	pub completed: bool,
}

#[derive(Deserialize, Serialize)]
pub struct Config{
	pub current_list: Option<String>,
	pub lists: Vec<String>,
	pub id: i32
}
