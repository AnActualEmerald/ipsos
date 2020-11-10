use clap::{App, Arg, ArgMatches, SubCommand};

//hiding this in its own file cus its ugly
pub fn get_matches<'a>() -> ArgMatches<'a> {
	App::new("Ipsos Watchlist Manager")
        .version("0.1.0")
        .author("Emerald (https://discord.gg/KwzhFaK)")
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
                    Arg::with_name("NAME")
                        .help("Name of the new watchlist")
                        .required(true),
                )
                .arg(
                    Arg::with_name("switch")
                        .help("Switch to the watchlist after creating it")
                        .long("switch")
                        .short("s"),
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
                    Arg::with_name("TITLE")
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
        .subcommand(
            SubCommand::with_name("update")
                .alias("up")
                .about("Update a show in the watchlist")
                .after_help("Running this command with no options will add 1 to the watched episode counter of the current show")
                .arg(Arg::with_name("TITLE").help(
                    "The title of the show to update. Will update the current show if omitted",
                ))
                .arg(
                    Arg::with_name("done")
                        .help("Whether or not you've finished the show")
                        .long("done")
                        .short("d"),
                )
                .arg(
                    Arg::with_name("watched")
                        .help("How many episodes you've watched. Adds to the current total and can be negative. Defaults to +1. Negative values need to use the -w=value form")
                        .default_value("1")
                        .takes_value(true)
                        .long("watched")
                        .short("w")
                ).arg(Arg::with_name("length")
                        .help("How many episodes the show has")
                        .takes_value(true)
                        .long("length")
                        .short("l"))
        ).subcommand(SubCommand::with_name("watch")
                .alias("w")
                .about("Watch a show from the watchlist")
                .arg(Arg::with_name("TITLE").help("Title of the show to watch").required(true)))
        .get_matches()
}

// pub fn make_yaml_app() -> App {
//     clap::lo
// }
