use clap::{App, Arg};

fn main() {
    let matches = App::new("Repertorre")
        .version("0.1.0")
        .author("Miles Olson <mpolson64@gmail.com>")
        .about("A tool to manage and review chess opening repertoires")
        .subcommand(App::new("ingest")
            .about("Ingest new lines into the database")
            .arg(Arg::new("DB")
                .required(true)
                .about("Sets the database file to save to"))
            .arg(Arg::new("PGN")
                .required(true)
                .about("PGN of the line to ingest")))
        .subcommand(App::new("export")
            .about("Export database file to PGN")
            .arg(Arg::new("DB")
                .required(true)
                .about("Sets the database file to export")))
        .subcommand(App::new("analyze")
            .about("See where a game deviated from repertoire and show possible continuations")
            .arg(Arg::new("DB")
                .required(true)
                .about("Sets the database file to analyze against"))
            .arg(Arg::new("PGN")
                .required(true)
                .about("PGN of the line to analyze"))
            .arg(Arg::new("NUM_CONTINUATIONS")
                .required(false)
                .about("The number of coninuations to show (default 1)")))
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("ingest") {
        let db_name = matches.value_of("DB").unwrap();
        let pgn_name = matches.value_of("PGN").unwrap();

        println!("Ingesting {} into {}", pgn_name, db_name);
    } else if let Some(matches) = matches.subcommand_matches("export") {
        let db_name = matches.value_of("DB").unwrap();

        println!("Exporting {}", db_name);
    } else if let Some(matches) = matches.subcommand_matches("analyze") {
        let default_num_continuations = 1;

        let db_name = matches.value_of("DB").unwrap();
        let pgn_name = matches.value_of("PGN").unwrap();
        let num_continuations = matches.value_of("NUM_CONTINUATIONS")
            .and_then(|val| Some(val.parse::<u32>().unwrap_or(default_num_continuations)))
            .unwrap_or(default_num_continuations);

        println!("Analyzing {} against {} showing {} continuations", pgn_name, db_name, num_continuations);
    }
}
