extern crate clap;

use clap::{App, Arg, SubCommand};

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
        println!("{:?}", matches.value_of("NAME").unwrap());
    }
}
