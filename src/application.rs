use clap::{App, Arg, ArgMatches, SubCommand};

//hiding this in its own file cus its ugly
pub fn get_matches<'a>() -> ArgMatches<'a> {
    App::new("Ipsos Watchlist Manager")
        .version("0.0.1")
        .author("Emerald")
        .about("Manage your watchlists with ease from the command line")
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .about("Show all the current watchlists")
                .arg(
                    Arg::with_name("shows")
                        .short("s")
                        .long("shows")
                        .takes_value(false)
                        .help("Show detailed information on the shows in the watchlist"),
                ),
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
        .subcommand(
            SubCommand::with_name("add")
                .alias("a")
                .about("Add a show to the watchlist")
                .arg(
                    Arg::with_name("title")
                        .short("t")
                        .long("title")
                        .help("The title of the show to add")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("len")
                        .takes_value(true)
                        .help("How many episodes the show has"),
                )
                .arg(
                    Arg::with_name("watched")
                        .short("w")
                        .long("watched")
                        .takes_value(true)
                        .help("How many episodes you've already watched")
                        .requires("length"),
                )
                .arg(
                    Arg::with_name("done")
                        .short("d")
                        .long("done")
                        .takes_value(false)
                        .help("Have you already finished the show?"),
                ),
        )
        .get_matches()
}
