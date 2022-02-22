use clap::{Command, Arg};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    // TODO: this thing compiles the home directory of the user at compile time
    let matches = get_matches();
    let data_dir = dirs::data_dir().expect("Could not find data directory");
    let store = sled::open(data_dir.join("kv.db"))?;

    match matches.subcommand() {
        Some(("set", submatches)) => {
            let key = submatches.value_of("key").unwrap();
            let val = submatches.value_of("val").unwrap();
            store.insert(key, val)?;
            println!("set the key");
        }

        Some(("get", submatches)) => {
            let key = submatches.value_of("key").unwrap();
            if let Some(val) = store.get(key)? {
                let s = String::from_utf8(val.to_vec())?;
                println!("{}", s);
            }
        }
        _ => println!("No subcommand was used"),
    }

    Ok(())
}

fn get_matches() -> clap::ArgMatches {
    Command::new("kv")
        .about("A simple key-value store")
        .subcommand(
            Command::new("set")
            .about("Sets a key/value pair")
            .arg(Arg::new("key")
                .help("key to set")
                .value_name("KEY")
                 .required(true)
                 )
            .arg(Arg::new("val")
                .help("value to set")
                .value_name("VAL")
                .multiple_values(true)
                 .required(true)
                 )
            )
        .subcommand(
            Command::new("get")
            .about("Gets a value for a given key")
            .arg(Arg::new("key")
                .help("key to fetch")
                .value_name("KEY")
                 .required(true)
                 )
            )
        .subcommand_required(true)
        .get_matches()
}
