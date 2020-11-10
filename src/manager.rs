use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{Error, Read, Write};
use std::path::PathBuf;
use prettytable::{Table, format};

pub fn update_show(
	title: Option<&str>,
	watched: Option<&str>,
	len: Option<&str>,
	done: bool,
) -> Result<(), String> {
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

pub fn watch_show(title: &str) -> Result<(), String> {
	let path = gen_path();
	let mut ml = read_json(&path)?;

	ml.lists.entry(ml.current.clone()).and_modify(|v| {
		v.current = title.to_owned();
	});

	save_json(&ml, &path)?;

	println!("Now watching {}", title);
	Ok(())
}

pub fn add_show(
	title: Option<&str>,
	len: Option<&str>,
	watched: Option<&str>,
	completed: bool,
) -> Result<(), String> {
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

pub fn load_list(name: &str) -> Result<(), String> {
	let path = gen_path();
	let mut ml = read_json(&path)?;
	ml.current = name.to_owned();
	save_json(&ml, &path)?;
	println!("Switched to list {}", name);
	Ok(())
}

pub fn list_shows() -> Result<(), String> {
	let ml = read_json(&gen_path())?;
	let current = ml.get_current()?;
	let mut table = Table::new();
	let format = format::FormatBuilder::new()
		.column_separator('|')
		.borders('|')
		.padding(1, 1)
		.separators(&[format::LinePosition::Top, format::LinePosition::Intern,format::LinePosition::Bottom], format::LineSeparator::new('-', '+', '+', '+'))
		.separator(format::LinePosition::Title, format::LineSeparator::new('=', '+', '+', '+'))
		.build();
	table.set_format(format);
	table.set_titles(row!["", "Title", "% watched", "(watched/length)"]);
	if let Some(s) = current.get_current(){
		// table.add_row(row![" > ", s.title, format!("{}%", ((s.watched as f32 / s.length as f32) * 100.0) as i32), format!("({}/{})", s.watched, s.length)]);
		// println!(
		// 	"\n*{}*:\n    Watched: {}%({}/{})\n    Completed: {}\n",
		// 	s.title,
		// 	((s.watched as f32 / s.length as f32) * 100.0) as i32,
		// 	s.watched,
		// 	s.length,
		// 	s.completed
		// );
	}

	current.shows.iter().for_each(|(_, v)| {
		

		// let res = format!(
		// 	"{}:\n    Watched: {}%({}/{})\n    Completed: {}",
		// 	v.title,
		// 	((v.watched as f32 / v.length as f32) * 100.0) as i32,
		// 	v.watched,
		// 	v.length,
		// 	v.completed
		// );
		if current.current == v.title {
			// println!("\nCurrently watching:\n{}\n\n", res);
			table.add_row(row![">", v.title, format!("{}%", ((v.watched as f32 / v.length as f32) * 100.0) as i32), format!("({}/{})", v.watched, v.length)]);

		} else {
			// println!("{}\n", res);
			table.add_row(row!["", v.title, format!("{}%", ((v.watched as f32 / v.length as f32) * 100.0) as i32), format!("({}/{})", v.watched, v.length)]);
		}
	});

	table.printstd();

	Ok(())
}

pub fn list_lists() -> Result<(), String> {
	let ml = read_json(&gen_path())?;
	println!(
		"Current: {}\nShows: {}\n\nLists: {}",
		ml.current,
		ml.get_current()?.list(),
		ml.list()
	);
	Ok(())
}

pub fn new_watchlist(name: &str) -> Result<(), String> {
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
pub fn save_json(data: &MainList, path: &PathBuf) -> Result<(), String> {
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

pub fn read_json(path: &PathBuf) -> Result<MainList, String> {
	let data: MainList;
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

pub fn gen_path() -> PathBuf {
	let r_path = format!("{}/.ipsos/lists.json", dirs::home_dir().unwrap().display()); //I want this to be in the user's home directory
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
pub struct Show {
	pub title: String,
	pub length: i32,
	pub watched: i32,
	pub completed: bool,
}
