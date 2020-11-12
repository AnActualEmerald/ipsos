use clap::{App, Arg, ArgMatches, SubCommand};

//hiding this in its own file cus its ugly
pub fn get_matches<'a>() -> ArgMatches<'a> {
	App::new("Ipsos Watchlist Manager")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Emerald <https://discord.gg/nrvRnkVmJm>")
        .about("Manage your watchlists with ease from the command line")
        .subcommand(
            SubCommand::with_name("list")
                .alias("l")
                .about("Show all the shows in the current watchlist")
                .arg(
                    Arg::with_name("lists")
                        .short("l")
                        .long("lists")
                        .takes_value(false)
                        .help("Lists the watchlists"),
                ),
        )
        .subcommand(
            SubCommand::with_name("new")
                .alias("n")
                .about("Create a new watchlist")
                .arg(
                    Arg::with_name("switch")
                        .help("Switch to the watchlist after creating it")
                        .long("switch")
                        .short("s"),
                )
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
                .arg(Arg::with_name("imdb")
                    .help("Search IMDb for a show")
                    .short("i")
                    .long("imdb")
                    .takes_value(false),
                )
                .arg(
                    Arg::with_name("length")
                        .short("l")
                        .long("len")
                        .takes_value(true)
                        .help("How many episodes the show has"),
                )
                .arg(
                    Arg::with_name("done")
                        .short("d")
                        .long("done")
                        .takes_value(false)
                        .help("Have you already finished the show?"),
                )
                .arg(
                    Arg::with_name("TITLE")
                        .multiple(true)
                        .help("The title of the show to add")
                        .takes_value(true)
                        .required(true),
                ),
        ).subcommand(SubCommand::with_name("watch")
                .alias("w")
                .about("Watch a show from the watchlist")
                .arg(Arg::with_name("ID").help("ID of the show to watch").required(true))
        ).subcommand(SubCommand::with_name("remove")
                .alias("r")
                .alias("rem")
                .about("Remove a show from the current watch list")
                .arg(Arg::with_name("title").long("title").short("t").takes_value(true).help("Remove show by title rather than ID"))
                .arg(Arg::with_name("ID").help("The id of the show to be removed").takes_value(true).required_unless("title"))
        ).subcommand(SubCommand::with_name("delete")
                .alias("d")
                .alias("del")
                .about("Delete an entire watchlist")
                .arg(Arg::with_name("NAME").takes_value(true).required(true).help("The name of the watchlist to be deleted"))
        )   
        
        .get_matches()
}
