use dirs;
use std::fs;
use std::path::Path;

fn main() {
	let path = Path::new(format!("{}/ipsos/config.toml", dirs::config_dir().unwrap()));

	if path.exists() {
		println!("Config file exists, exiting build script");
		return;
	} else {
		println!("Creating default config...");
		fs::write(path, include_str!("config.toml.ex")).expect("Unable to create config file");
		println!("Done. ");
	}
}
